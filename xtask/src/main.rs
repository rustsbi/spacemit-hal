mod elf2bin;
mod fastboot;
mod fsbl;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(about = "Build and run K1/M1 development FSBL images")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Wrap a raw FSBL binary in a K1/M1 container.
    WrapFsbl {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Convert an ELF, wrap it, then stage and continue via Fastboot.
    Run {
        elf: PathBuf,
        #[arg(short, long)]
        serial: Option<String>,
    },
}

fn output_path(input: &std::path::Path) -> PathBuf {
    let mut name = input.file_stem().unwrap_or_default().to_os_string();
    name.push("-FSBL.bin");
    input.with_file_name(name)
}

fn write_image(raw: &[u8], output: &std::path::Path) -> Result<()> {
    let image = fsbl::wrap(raw)?;
    fs::write(output, &image).with_context(|| format!("write {}", output.display()))?;
    println!("{}: {} bytes", output.display(), image.len());
    Ok(())
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::WrapFsbl { input, output } => {
            let raw = fs::read(&input).with_context(|| format!("read {}", input.display()))?;
            let output = output.unwrap_or_else(|| output_path(&input));
            anyhow::ensure!(
                !output.exists() || fs::canonicalize(&input)? != fs::canonicalize(&output)?,
                "output must not overwrite the input"
            );
            write_image(&raw, &output)
        }
        Command::Run { elf, serial } => {
            let bytes = fs::read(&elf).with_context(|| format!("read {}", elf.display()))?;
            let raw = elf2bin::convert(&bytes)?;
            let output = output_path(&elf);
            write_image(&raw, &output)?;
            fastboot::run(&output, serial.as_deref())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_and_names() {
        Cli::command().debug_assert();
        assert!(Cli::try_parse_from(["xtask", "run", "hello.elf"]).is_ok());
        assert!(Cli::try_parse_from(["xtask", "wrap-fsbl", "hello.bin"]).is_ok());
        assert_eq!(
            output_path(std::path::Path::new("hello.elf")),
            PathBuf::from("hello-FSBL.bin")
        );
        assert_eq!(
            output_path(std::path::Path::new("hello")),
            PathBuf::from("hello-FSBL.bin")
        );
    }

    use clap::CommandFactory;
}
