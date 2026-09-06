use anyhow::{Context, Result, ensure};
use std::{ffi::OsStr, path::Path, process::Command};

pub fn run(image: &Path, serial: Option<&str>) -> Result<()> {
    transfer(image, serial, |args| {
        let output = Command::new("fastboot").args(args).output().map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                anyhow::anyhow!("fastboot not found; install Android SDK Platform-Tools and add it to PATH: https://developer.android.com/tools/releases/platform-tools")
            } else { error.into() }
        })?;
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        ensure!(
            output.status.success(),
            "fastboot failed: {}",
            output.status
        );
        Ok(output.stdout)
    })
}

fn transfer(
    image: &Path,
    serial: Option<&str>,
    mut execute: impl FnMut(&[&OsStr]) -> Result<Vec<u8>>,
) -> Result<()> {
    let devices = execute(&[OsStr::new("devices")])?;
    let devices = std::str::from_utf8(&devices).context("invalid fastboot device list")?;
    let devices: Vec<_> = devices
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let serial = fields.next()?;
            (fields.next()? == "fastboot").then_some(serial)
        })
        .collect();
    let serial = if let Some(serial) = serial {
        ensure!(devices.contains(&serial), "device {serial} not found");
        serial
    } else {
        ensure!(
            !devices.is_empty(),
            "no Fastboot device; connect the board in ROM download mode"
        );
        ensure!(
            devices.len() == 1,
            "multiple Fastboot devices; select one with --serial"
        );
        devices[0]
    };
    execute(&[
        OsStr::new("-s"),
        OsStr::new(serial),
        OsStr::new("stage"),
        image.as_os_str(),
    ])?;
    execute(&[OsStr::new("-s"), OsStr::new(serial), OsStr::new("continue")])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_and_stage_failure() {
        for fail in [false, true] {
            let mut calls = Vec::new();
            let result = transfer(Path::new("hello world-FSBL.bin"), None, |args| {
                calls.push(
                    args.iter()
                        .map(|s| s.to_string_lossy().into_owned())
                        .collect::<Vec<_>>(),
                );
                if calls.len() == 1 {
                    return Ok(b"board\tfastboot\n".to_vec());
                }
                ensure!(!fail, "stage failed");
                Ok(Vec::new())
            });
            assert_eq!(result.is_err(), fail);
            assert_eq!(calls.len(), if fail { 2 } else { 3 });
            assert_eq!(calls[1], ["-s", "board", "stage", "hello world-FSBL.bin"]);
            if !fail {
                assert_eq!(calls[2], ["-s", "board", "continue"]);
            }
        }
    }

    #[test]
    fn refuses_ambiguous_or_missing_devices() {
        for list in ["", "a fastboot\nb fastboot\n"] {
            let mut calls = 0;
            assert!(
                transfer(Path::new("image"), None, |_| {
                    calls += 1;
                    Ok(list.as_bytes().to_vec())
                })
                .is_err()
            );
            assert_eq!(calls, 1);
        }
    }
}
