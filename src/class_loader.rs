use std::fmt::Debug;

use colored::Colorize;

use crate::{constants::Constant, utilities::ByteShifter};

pub struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<Constant>,
}

impl ClassFile {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut byte_shifter = ByteShifter::new(bytes);

        // The first 4 bytes is just the `0xCAFEBABE` easter egg
        byte_shifter.skip_bytes(4);

        let minor_version = byte_shifter.read_2_bytes();
        let major_version = byte_shifter.read_2_bytes();

        // Now we have to extract the constants
        let constant_pool_size = byte_shifter.read_2_bytes();
        let mut constant_pool: Vec<Constant> = Vec::new();
        for _ in 1..constant_pool_size {
            let tag = byte_shifter.read_byte();

            constant_pool.push(Constant::from_tag(tag, &mut byte_shifter));
        }

        Self {
            minor_version,
            major_version,
            constant_pool,
        }
    }
}

impl Debug for ClassFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "minor version: {}, major version: {}",
            self.minor_version, self.major_version
        )?;

        writeln!(f, "--- Constant Pool ---")?;

        for (i, constant) in self.constant_pool.iter().enumerate() {
            writeln!(f, "Id: {}, {:?}", (i + 1).to_string().green(), constant)?;
        }

        Ok(())
    }
}
