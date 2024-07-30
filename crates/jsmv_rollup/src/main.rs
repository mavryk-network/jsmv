use std::{env, fs, path::PathBuf, str::FromStr};

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use derive_more::{Deref, DerefMut};
use figment::{
    providers::{Env, Format, Json},
    Figment,
};
use jsmv_rollup::{
    deploy_ctez_contract, rollup, BootstrapAccount, BridgeContract, JsmvRollup,
};
use mavkit::{MavkitClient, MavkitRollupNode, MavkitThread};
use serde::{Deserialize, Serialize};
use mavryk_crypto_rs::hash::{ContractKt1Hash, ContractMv1Hash, SmartRollupHash};

const JSMV_ROLLUP_OPERATOR_ALIAS: &str = "jsmv_rollup_operator";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    mavkit_client_bin: Option<PathBuf>,
    mavkit_client_dir: PathBuf,
    mavkit_node_endpoint: String,
    mavkit_rollup_node_bin: Option<PathBuf>,
    mavkit_rollup_node_dir: PathBuf,
}

impl Config {
    fn mavkit_client(&self) -> MavkitClient {
        MavkitClient {
            mavkit_client_bin: self.mavkit_client_bin.clone(),
            mavkit_client_dir: self.mavkit_client_dir.clone(),
            endpoint: self.mavkit_node_endpoint.clone(),
            disable_disclaimer: true,
        }
    }

    fn mavkit_rollup_node(&self) -> MavkitRollupNode {
        MavkitRollupNode {
            mavkit_rollup_node_bin: self.mavkit_rollup_node_bin.clone(),
            mavkit_rollup_node_dir: self.mavkit_rollup_node_dir.clone(),
            mavkit_client_dir: self.mavkit_client_dir.clone(),
            endpoint: self.mavkit_node_endpoint.clone(),
        }
    }
}

#[derive(Debug, Clone, Deref, DerefMut)]
struct Alias(String);

impl From<Option<String>> for Alias {
    fn from(value: Option<String>) -> Self {
        Self(value.unwrap_or(JSMV_ROLLUP_OPERATOR_ALIAS.to_string()))
    }
}

impl ToString for Alias {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
enum Mv1AddressOrAlias {
    Address(ContractMv1Hash),
    Alias(String),
}

impl Mv1AddressOrAlias {
    fn as_alias(&self) -> Option<&str> {
        match self {
            Self::Address(_) => None,
            Self::Alias(alias) => Some(alias),
        }
    }
}

impl FromStr for Mv1AddressOrAlias {
    type Err = anyhow::Error;

    fn from_str(address_or_alias: &str) -> Result<Self> {
        if address_or_alias.starts_with("mv1") {
            // SAFETY: address_or_alias is known to be a mv1 address
            Ok(Self::Address(address_or_alias.parse()?))
        } else {
            Ok(Self::Alias(address_or_alias.to_string()))
        }
    }
}

impl ToString for Mv1AddressOrAlias {
    fn to_string(&self) -> String {
        match self {
            Self::Address(address) => address.to_base58_check(),
            Self::Alias(alias) => alias.clone(),
        }
    }
}

#[derive(Debug, Clone, Deref, DerefMut)]
struct OperatorAddress(ContractMv1Hash);

impl TryFrom<Option<ContractMv1Hash>> for OperatorAddress {
    type Error = anyhow::Error;

    fn try_from(value: Option<ContractMv1Hash>) -> Result<Self> {
        let address = match value {
            Some(address) => address,
            None => env::var("JSMV_ROLLUP_OPERATOR_ADDRESS").map_err(|_| {
                anyhow!("Missing address. Please set JSMV_ROLLUP_ADDRESS or pass --address <ADDRESS>")
            })?.parse()?,
        };

        Ok(Self(address))
    }
}

#[derive(Debug, Clone, Deref, DerefMut)]
struct Operator(Mv1AddressOrAlias);

impl TryFrom<Option<Mv1AddressOrAlias>> for Operator {
    type Error = anyhow::Error;

    fn try_from(address_or_alias: Option<Mv1AddressOrAlias>) -> Result<Self> {
        match address_or_alias {
            Some(address_or_alias) => Ok(Self(address_or_alias)),
            None => {
                if let Ok(address) = env::var("JSMV_ROLLUP_OPERATOR_ADDRESS") {
                    return Ok(Self(Mv1AddressOrAlias::Address(address.parse()?)));
                }

                if let Ok(alias) = env::var("JSMV_ROLLUP_OPERATOR") {
                    return Ok(Self(Mv1AddressOrAlias::Alias(alias)));
                }

                Ok(Self(Mv1AddressOrAlias::Alias(
                    JSMV_ROLLUP_OPERATOR_ALIAS.to_string(),
                )))
            }
        }
    }
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum OperatorCommand {
    GenKeys {
        #[arg(long, value_name = "ALIAS")]
        /// Alias for the operator key
        alias: Option<String>,
    },
    Info {
        #[arg(long, value_name = "ADDRESS|ALIAS")]
        /// mv1 for the operator key
        operator: Option<Mv1AddressOrAlias>,
    },
    ImportKeys {
        #[arg(long, value_name = "ALIAS")]
        /// Alias for the operator key
        alias: Option<String>,
        #[arg(long, value_name = "SECRET_KEY")]
        /// Path to the secret key file
        secret_key: String,
    },
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(subcommand)]
    Operator(OperatorCommand),

    MakeInstaller {
        #[arg(long, value_name = "PATH")]
        /// Path to the kernel .wasm file
        kernel: PathBuf,
        #[arg(long, value_name = "ADDRESS")]
        /// KT1 address of the bridge contract
        bridge: ContractKt1Hash,
        #[arg(long, value_name = "PATH")]
        /// Path to the installer output folder
        output: PathBuf,
    },
    DeployBridge {
        #[arg(long, value_name = "ADDRESS")]
        /// mv1 address of the operator
        operator: Option<ContractMv1Hash>,
        #[arg(long, value_name = "PATH")]
        /// Path to the bootstrap accounts file
        bootstrap_accounts: Option<PathBuf>,
    },
    Deploy {
        #[arg(long, value_name = "ADDRESS|ALIAS")]
        /// mv1 address/alias of the operator
        operator: Option<Mv1AddressOrAlias>,
        #[arg(long, value_name = "PATH")]
        /// Path to the kernel .wasm file
        kernel: PathBuf,
        #[arg(long, value_name = "ADDRESS")]
        /// KT1 address of the bridge contract
        bridge: ContractKt1Hash,
        #[arg(long, value_name = "PATH")]
        /// Path to the installer output folder
        output: PathBuf,
    },
    DeployInstaller {
        #[arg(long, value_name = "ADDRESS|ALIAS")]
        /// mv1 address/alias of the operator
        operator: Option<Mv1AddressOrAlias>,
        #[arg(long, value_name = "PATH")]
        /// Path to the installer.wasm file
        installer: PathBuf,
        #[arg(long, value_name = "ADDRESS")]
        /// KT1 address of the bridge contract
        bridge: ContractKt1Hash,
    },
    Run {
        #[arg(long, value_name = "ADDRESS|ALIAS")]
        /// mv1 address/alias of the operator
        operator: Option<Mv1AddressOrAlias>,
        #[arg(long, value_name = "PATH")]
        /// Path to the preimages directory
        preimages: PathBuf,
        #[arg(long, value_name = "PATH")]
        /// Path to the logs directory
        logs: PathBuf,
        #[arg(long, value_name = "ADDRESS")]
        /// Rollup address
        rollup: SmartRollupHash,
        /// Address to bind the rollup node to
        #[arg(long, value_name = "IP", default_value = "127.0.0.1")]
        addr: String,
        /// Port to run the rollup node on
        #[arg(long, value_name = "PORT", default_value = "8932")]
        port: u16,
    },
}

fn gen_keys(cfg: &Config, alias: Option<String>) -> Result<()> {
    let client = cfg.mavkit_client();
    let alias = Alias::from(alias);

    client.gen_keys(&alias)?;
    println!("Generated keys for {}", alias.to_string());

    Ok(())
}

fn info(cfg: &Config, operator: Option<Mv1AddressOrAlias>) -> Result<()> {
    let client = cfg.mavkit_client();

    let operator = Operator::try_from(operator)?;

    if let Some(alias) = operator.as_alias() {
        let info = client.alias_info(alias)?;

        println!("Address: {}", info.address);
        println!("Public Key: {}", info.public_key);
        println!("Secret Key: {}", info.secret_key);
    }

    let balance = client.balance(&operator.to_string())?;
    println!("Balance: {} ꜩ", balance);

    Ok(())
}

fn import_keys(cfg: &Config, alias: Option<String>, secret_key: &str) -> Result<()> {
    let client = cfg.mavkit_client();
    let alias = Alias::from(alias);

    print!("Importing key for {}...", alias.to_string());
    client.import_secret_key(&alias, secret_key)?;
    println!(" done");

    Ok(())
}

fn make_installer(
    kernel: PathBuf,
    bridge: ContractKt1Hash,
    output: PathBuf,
) -> Result<()> {
    let bridge = BridgeContract::from(bridge);

    print!("Building installer...");

    let installer = rollup::make_installer(&kernel, &output.join("preimages"), &bridge)?;
    fs::write(output.join("installer.wasm"), installer)?;

    println!(" done");

    Ok(())
}

fn deploy_bridge(
    cfg: &Config,
    operator: Option<ContractMv1Hash>,
    bootstrap_accounts: Option<PathBuf>,
) -> Result<()> {
    let client = cfg.mavkit_client();
    let operator = OperatorAddress::try_from(operator)?;

    let bootstrap_accounts = match bootstrap_accounts {
        Some(bootstrap_accounts) => {
            let bootstrap_accounts = fs::read_to_string(bootstrap_accounts)?;
            serde_json::from_str::<Vec<BootstrapAccount>>(&bootstrap_accounts)?
        }
        None => vec![],
    };

    let ctez_address =
        deploy_ctez_contract(&client, &operator.to_string(), bootstrap_accounts.iter())?;

    let bridge_address = BridgeContract::deploy(
        &client,
        &operator.to_string(),
        &ctez_address.to_string(),
    )?;

    println!("Bridge address: {}", bridge_address);

    Ok(())
}

fn deploy_installer(
    cfg: &Config,
    operator: Option<Mv1AddressOrAlias>,
    installer: PathBuf,
    bridge: ContractKt1Hash,
) -> Result<()> {
    let client = cfg.mavkit_client();
    let operator = Operator::try_from(operator)?;
    let bridge = BridgeContract::from(bridge);

    let installer = fs::read(installer)?;
    let rollup_address = JsmvRollup::deploy(&client, &operator.to_string(), &installer)?;
    bridge.set_rollup(&client, &operator.to_string(), &rollup_address)?;

    println!("{}", rollup_address);

    Ok(())
}

fn deploy(
    cfg: &Config,
    operator: Option<Mv1AddressOrAlias>,
    kernel: PathBuf,
    bridge: ContractKt1Hash,
    output: PathBuf,
) -> Result<()> {
    let client = cfg.mavkit_client();
    let operator = Operator::try_from(operator)?;
    let bridge = BridgeContract::from(bridge);

    print!("Building installer...");

    let installer = rollup::make_installer(&kernel, &output.join("preimages"), &bridge)?;
    fs::write(output.join("installer.wasm"), &installer)?;

    println!(" done");

    println!("Deploying rollup...");

    let rollup_address = JsmvRollup::deploy(&client, &operator.to_string(), &installer)?;
    bridge.set_rollup(&client, &operator.to_string(), &rollup_address)?;

    println!("\tAddress: {}", rollup_address);

    Ok(())
}

fn run(
    cfg: &Config,
    operator: Option<Mv1AddressOrAlias>,
    preimages: PathBuf,
    logs: PathBuf,
    rollup: SmartRollupHash,
    addr: String,
    port: u16,
) -> Result<()> {
    let rollup_node = cfg.mavkit_rollup_node();
    let rollup = JsmvRollup::from(rollup);
    let operator = Operator::try_from(operator)?;

    let child = rollup.run(
        &rollup_node,
        &operator.to_string(),
        &preimages,
        &logs,
        &addr,
        port,
    )?;
    let thread = MavkitThread::from_child(child);

    MavkitThread::join(vec![thread])?;

    Ok(())
}

fn default_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".jsmv")
        .join("rollup.json")
}

fn main() -> Result<()> {
    let cli: Cli = Cli::parse();

    // `make-installer` doesn't require the config file, hence it is hoisted
    if let Command::MakeInstaller {
        kernel,
        bridge,
        output,
    } = cli.command
    {
        return make_installer(kernel, bridge, output);
    }

    // all other commands require the config file are handled below
    let config: Config = Figment::new()
        .merge(Json::file(
            cli.config.clone().unwrap_or(default_config_path()),
        ))
        .merge(Env::prefixed("JSMV_ROLLUP_"))
        // TODO: Uncomment this once I've figured out how to merge optional CLI
        // flags with Figment
        // .merge(Serialized::defaults(cli))
        .extract()?;

    match cli.command {
        Command::Operator(OperatorCommand::GenKeys { alias }) => gen_keys(&config, alias),
        Command::Operator(OperatorCommand::Info { operator }) => info(&config, operator),
        Command::Operator(OperatorCommand::ImportKeys { alias, secret_key }) => {
            import_keys(&config, alias, &secret_key)
        }
        Command::DeployBridge {
            operator,
            bootstrap_accounts,
        } => deploy_bridge(&config, operator, bootstrap_accounts),
        Command::Deploy {
            operator,
            kernel,
            bridge,
            output,
        } => deploy(&config, operator, kernel, bridge, output),
        Command::Run {
            operator,
            preimages,
            logs,
            rollup,
            addr,
            port,
        } => run(&config, operator, preimages, logs, rollup, addr, port),
        Command::DeployInstaller {
            operator,
            installer,
            bridge,
        } => deploy_installer(&config, operator, installer, bridge),
        Command::MakeInstaller { .. } => {
            unreachable!(
                "`make-installer` is handled above and should never reach this point"
            )
        }
    }
}
