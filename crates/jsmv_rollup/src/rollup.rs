use std::{
    fmt::{self, Display},
    fs::{self, File},
    path::Path,
    process::Child,
};

use anyhow::Result;
use derive_more::{Deref, DerefMut};
use fs_extra::dir::CopyOptions;
use mavkit::{MavkitClient, MavkitRollupNode};
use mavryk_crypto_rs::hash::{ContractKt1Hash, SmartRollupHash};
use mavryk_smart_rollup_host::path::{OwnedPath, RefPath};
use mavryk_smart_rollup_installer::{
    installer, preimages, KERNEL_BOOT_PATH, PREPARE_KERNEL_PATH,
};
use mavryk_smart_rollup_installer_config::binary::owned::{
    OwnedBytes, OwnedConfigInstruction, OwnedConfigProgram,
};

use crate::BridgeContract;

const TICKETER_PATH: RefPath = RefPath::assert_from(b"/ticketer");

pub fn make_installer(
    kernel_file: &Path,
    preimages_dir: &Path,
    bridge_contract: &BridgeContract,
) -> Result<Vec<u8>> {
    let kernel = std::fs::read(kernel_file).map_err(preimages::Error::ContentFile)?;
    let root_hash = preimages::content_to_preimages(kernel, preimages_dir)?;

    let installer_program = OwnedConfigProgram(vec![
        // 1. Prepare kernel installer
        OwnedConfigInstruction::reveal_instr(
            root_hash,
            OwnedPath::from(PREPARE_KERNEL_PATH),
        ),
        OwnedConfigInstruction::move_instr(
            OwnedPath::from(PREPARE_KERNEL_PATH),
            OwnedPath::from(KERNEL_BOOT_PATH),
        ),
        // 2. Set `jsmv` ticketer as the bridge contract address
        OwnedConfigInstruction::set_instr(
            OwnedBytes(bincode::serialize(&ContractKt1Hash::from_base58_check(
                bridge_contract,
            )?)?),
            OwnedPath::from(TICKETER_PATH),
        ),
    ]);

    let installer = installer::with_config_program(installer_program);

    Ok(installer)
}

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
pub struct JsmvRollup(String);

impl Display for JsmvRollup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<SmartRollupHash> for JsmvRollup {
    fn from(hash: SmartRollupHash) -> Self {
        Self(hash.to_base58_check())
    }
}

impl JsmvRollup {
    pub fn deploy(
        client: &MavkitClient,
        operator: &str,
        installer: &[u8],
    ) -> Result<Self> {
        let address = client.originate_rollup(
            operator,
            "jsmv_rollup",
            "wasm_2_0_0",
            "(pair bytes (ticket unit))",
            &hex::encode(installer),
        )?;

        Ok(Self(address))
    }

    pub fn run(
        &self,
        rollup_node: &MavkitRollupNode,
        operator: &str,
        preimages_dir: &Path,
        logs_dir: &Path,
        addr: &str,
        port: u16,
    ) -> Result<Child> {
        let rollup_log_file = File::create(logs_dir.join("rollup.log"))?;

        // 1. Copy kernel installer preimages to rollup node directory
        let rollup_node_preimages_dir =
            rollup_node.mavkit_rollup_node_dir.join("wasm_2_0_0");

        fs::create_dir_all(&rollup_node_preimages_dir)?;
        fs_extra::dir::copy(
            preimages_dir,
            &rollup_node_preimages_dir,
            &CopyOptions {
                content_only: true,
                ..Default::default()
            },
        )?;

        // 2. Run the rollup node (configuring the kernel log file)
        rollup_node.run(
            addr,
            port,
            &rollup_log_file,
            &self.0,
            operator,
            &[
                "--log-kernel-debug",
                "--log-kernel-debug-file",
                logs_dir.join("kernel.log").to_str().expect("Invalid path"),
            ],
        )
    }
}
