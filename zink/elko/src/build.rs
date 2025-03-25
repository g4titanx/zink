//! Command `Build`.
use crate::utils::WasmBuilder;
use anyhow::{anyhow, Result};
use ccli::clap::{self, Parser};
use etc::{Etc, FileSystem};
use std::{env, fs, path::PathBuf};
use zinkc::{Compiler, Config};

/// Build zink project to EVM bytecode.
#[derive(Debug, Parser)]
#[command(name = "build", version)]
pub struct Build {
    /// The path of the cargo project.
    pub input: Option<PathBuf>,
    /// Write output to <filename>
    #[clap(short, long, value_name = "filename")]
    pub output: Option<PathBuf>,
    /// Write output to compiler-chosen filename in <dir>
    #[clap(long, value_name = "dir")]
    pub out_dir: Option<PathBuf>,
    /// Compiler configuration
    #[clap(flatten)]
    pub config: Config,
}

impl Build {
    /// Run build
    pub fn run(&self) -> Result<()> {
        // Get and check the input.
        let input = if let Some(input) = self.input.as_ref() {
            input.clone()
        } else {
            env::current_dir()?
        };
        {
            if Etc::new(&input)?.find("Cargo.toml").is_err() {
                return Ok(());
            }

            if !input.is_dir() {
                return Err(anyhow!(
                    "Only support rust project directory as input for now"
                ));
            }
        }

        // Build the wasm.
        let mut builder = WasmBuilder::new(input)?;
        {
            if let Some(out_dir) = self.out_dir.as_ref() {
                builder.with_out_dir(out_dir);
            }

            if let Some(output) = self.output.as_ref() {
                builder.with_output(output);
            }

            builder.build()?;
        }

        // Copy the WASM file to target/zink/
        let wasm_path = builder.output()?;
        let wasm_dest = wasm_path
            .with_extension("wasm")
            .parent()
            .unwrap()
            .join("zink")
            .join(wasm_path.file_name().unwrap());
        fs::create_dir_all(wasm_dest.parent().unwrap())?;
        fs::copy(&wasm_path, &wasm_dest)?;

        // Compile the wasm to evm bytecode.
        let wasm = fs::read(&wasm_path)?;
        let config = Config::default().dispatcher(self.config.dispatcher);
        let artifact = Compiler::new(config).compile(&wasm)?;
        let dst = wasm_path.with_extension("bin");

        fs::write(dst, artifact.runtime_bytecode)?;
        Ok(())
    }
}
