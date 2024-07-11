use std::sync::LazyLock;

use anyhow::Context;
use proc_mem::Process;

use crate::{error_to_cmd, CONFIG};

pub const PID: LazyLock<u32> = LazyLock::new(|| match CONFIG.pid {
    Some(pid) => pid,
    None => {
        let pid = match Process::with_name(PROCESS_NAME) {
            Ok(process) => *process.pid(),
            Err(e) => {
                error_to_cmd(&format!("Failed to find Elden Ring process: {:?}. Start the game first or specify the PID with the -p flag.", e));
                std::process::exit(1);
            }
        };

        pid
    }
});

pub static ELDENRING: LazyLock<Process> = LazyLock::new(|| match Process::with_pid(*PID) {
    Ok(process) => process,
    Err(e) => {
        error_to_cmd(&format!("Failed to open Elden Ring process: {:?}", e));
        std::process::exit(1);
    }
});

pub const TOUGHNESS_UPDATE_INTERVAL: std::time::Duration = std::time::Duration::from_millis(50);

pub static RESOLUTION: LazyLock<[i32; 2]> =
    LazyLock::new(|| match crate::get_resolution(&mut PID.to_owned()) {
        Some(resolution) => resolution,
        None => {
            error_to_cmd("Failed to get resolution. Make sure Elden Ring is running.");
            std::process::exit(1);
        }
    });

pub const _WINDOW_NAME: &str = "ELDEN RING™";
pub const PROCESS_NAME: &str = "eldenring.exe";
pub const MODULE_NAME: &str = "eldenring.exe";

pub const WORLD_CHR_MAN: LazyLock<usize> = LazyLock::new(|| {
    const SIGNATURE: &str = "48 8B 05 ?? ?? ?? ?? 48 85 C0 74 0F 48 39 88";

    let mut address = 0;

    match || -> anyhow::Result<()> {
        let module = ELDENRING
            .module(MODULE_NAME)
            .map_err(|e| anyhow::anyhow!("Failed to get module: {:?}", e))?;

        aobscan::PatternBuilder::from_ida_style(&SIGNATURE)
            .context("Failed to build pattern from IDA style signature")?
            .with_all_threads()
            .build()
            .scan(module.data(), |addrs: usize| {
                address = addrs;
                return true;
            });

        const OFFSET: usize = 3;
        const ADDITIONAL: usize = 7;

        address = ELDENRING.process_base_address + address;

        address = address
            + ELDENRING
                .read_mem::<i32>(address + OFFSET)
                .map_err(|e| anyhow::anyhow!("Failed to read memory: {:?}", e))?
                as usize
            + ADDITIONAL;

        Ok(())
    }() {
        Ok(_) => {}
        Err(e) => {
            error_to_cmd(&format!("Failed to find WorldChrMan address: {:?}. Most likely the game was updated and the signature is outdated.", e));
            std::process::exit(1);
        }
    }

    address
});

pub const LOCAL_PLAYER_OFFSET: usize = 0x10EF8;
