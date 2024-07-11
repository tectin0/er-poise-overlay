use anyhow::Context;

use crate::statics::{ELDENRING, LOCAL_PLAYER_OFFSET, WORLD_CHR_MAN};

pub fn get_toughness() -> anyhow::Result<f32> {
    let mut address = *WORLD_CHR_MAN;

    let mut value = read_memory::<usize>(address)? as usize;

    const OFFSETS: [usize; 5] = [LOCAL_PLAYER_OFFSET, 0x0, 0x190, 0x48, 0x10];

    let mut toughness = 0.0;

    for (i, offset) in OFFSETS.iter().enumerate() {
        address = value + *offset;
        match i {
            4 => {
                toughness = read_memory::<f32>(address).context(format!(
                    "Failed to read toughness from memory at address: 0x{:X}",
                    address
                ))?
            }
            _ => {
                value = read_memory::<usize>(address).context(format!(
                    "Failed to read memory at address : 0x{:X} at step :{}",
                    address, i
                ))? as usize
            }
        }
    }

    Ok(toughness)
}

fn read_memory<T: Default>(address: usize) -> anyhow::Result<T> {
    match ELDENRING.read_mem::<T>(address) {
        Ok(value) => Ok(value),
        Err(e) => Err(anyhow::anyhow!("Failed to read memory: {:?}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_toughness() {
        let toughness = get_toughness().unwrap();
        dbg!(toughness);
    }
}
