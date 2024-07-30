use std::{
    fs::File,
    path::PathBuf,
    process::{Child, Command, Stdio},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{path_or_default, run_command};

#[derive(Debug, Serialize, Deserialize)]
pub struct MavkitNode {
    /// Path to the mavkit-node binary
    /// If None, the binary will inside PATH will be used
    pub mavkit_node_bin: Option<PathBuf>,
    /// Path to the mavkit-node directory
    pub mavkit_node_dir: PathBuf,
}

impl MavkitNode {
    fn command(&self) -> Command {
        Command::new(path_or_default(
            self.mavkit_node_bin.as_ref(),
            "mavkit-node",
        ))
    }

    pub fn config_init(
        &self,
        network: &str,
        http_endpoint: &str,
        rpc_endpoint: &str,
        num_connections: u32,
    ) -> Result<()> {
        run_command(self.command().args([
            "config",
            "init",
            "--network",
            network,
            "--data-dir",
            self.mavkit_node_dir.to_str().expect("Invalid path"),
            "--net-addr",
            http_endpoint,
            "--rpc-addr",
            rpc_endpoint,
            "--connections",
            num_connections.to_string().as_str(),
        ]))
    }

    pub fn generate_identity(&self) -> Result<()> {
        run_command(self.command().args([
            "identity",
            "generate",
            "--data-dir",
            self.mavkit_node_dir.to_str().expect("Invalid path"),
        ]))
    }

    pub fn run(&self, log_file: &File, options: &[&str]) -> Result<Child> {
        Ok(self
            .command()
            .args([
                "run",
                "--data-dir",
                self.mavkit_node_dir.to_str().expect("Invalid path"),
            ])
            .args(options)
            .stdout(Stdio::from(log_file.try_clone()?))
            .stderr(Stdio::from(log_file.try_clone()?))
            .spawn()?)
    }
}
