use std::fmt::Debug;

use colored::Colorize;

use crate::{
    access_flags::AccessFlags,
    constants::{ClassAddress, Constant},
    utilities::ByteShifter,
};

pub struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<Constant>,
    access_flags: AccessFlags,

    /// An address that points to a `Class` constant that defines this specific class
    this_class_address: ClassAddress,

    /// An address that points to a `Class` that defines the superclass of this specific
    /// class, if it's None then it means that this class is the Object class
    super_class_address: Option<ClassAddress>,

    /// An array of interfaces that this specific class implements
    interfaces: Vec<ClassAddress>,
}

impl ClassFile {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut byte_shifter = ByteShifter::new(bytes);

        // The first 4 bytes is just the `0xCAFEBABE` easter egg
        byte_shifter.skip_bytes(4);

        let minor_version = byte_shifter.read_2_bytes();
        let major_version = byte_shifter.read_2_bytes();
        let constant_pool = read_constant_pool(&mut byte_shifter);
        let access_flags = AccessFlags::new(byte_shifter.read_2_bytes());
        let this_class_address = byte_shifter.read_2_bytes();
        let super_class_address = read_super_class_address(&mut byte_shifter);
        let interfaces = read_interfaces(&mut byte_shifter);

        Self {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class_address,
            super_class_address,
            interfaces,
        }
    }
}

fn read_constant_pool(byte_shifter: &mut ByteShifter) -> Vec<Constant> {
    let constant_pool_size = byte_shifter.read_2_bytes();
    let mut constant_pool: Vec<Constant> = Vec::new();

    // Constant pool starts at 1 for some reason
    for _ in 1..constant_pool_size {
        let tag = byte_shifter.read_byte();
        constant_pool.push(Constant::from_tag(tag, byte_shifter));
    }

    constant_pool
}

fn read_interfaces(byte_shifter: &mut ByteShifter) -> Vec<ClassAddress> {
    let interfaces_size = byte_shifter.read_2_bytes();
    let mut interfaces: Vec<ClassAddress> = Vec::new();

    for _ in 0..interfaces_size {
        interfaces.push(byte_shifter.read_2_bytes());
    }

    interfaces
}

fn read_super_class_address(byte_shifter: &mut ByteShifter) -> Option<ClassAddress> {
    let address = byte_shifter.read_2_bytes();

    // If address equals zero that means we are working with the Object class
    match address == 0 {
        false => None,
        true => Some(address),
    }
}

impl Debug for ClassFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print_details(self, f)?;

        if self.interfaces.len() > 0 {
            write!(f, "\n")?;
            print_interfaces(self, f)?;
        }

        if self.constant_pool.len() > 0 {
            write!(f, "\n")?;
            print_constant_pool(self, f)?;
        }

        // --- Utility printing functions ---
        fn print_details(
            class_file: &ClassFile,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            writeln!(
                f,
                "minor version: {}, major version: {}",
                class_file.minor_version, class_file.major_version
            )?;

            writeln!(f, "access flags: {:?}", class_file.access_flags)?;

            writeln!(
                f,
                "this class: {}, super class: {:?}",
                class_file.this_class_address, class_file.super_class_address
            )
        }

        fn print_interfaces(
            class_file: &ClassFile,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            writeln!(f, "--- Interfaces ---")?;
            for (i, interface) in class_file.interfaces.iter().enumerate() {
                writeln!(
                    f,
                    "Id: {}, Class index: {}",
                    (i + 1).to_string().green(),
                    interface.to_string().blue()
                )?;
            }

            Ok(())
        }

        fn print_constant_pool(
            class_file: &ClassFile,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            writeln!(f, "--- Constant Pool ---")?;
            for (i, constant) in class_file.constant_pool.iter().enumerate() {
                writeln!(f, "Id: {}, {:?}", (i + 1).to_string().green(), constant)?;
            }

            Ok(())
        }

        Ok(())
    }
}
