use std::fmt::Debug;

use colored::Colorize;

use crate::utilities::ByteShifter;

pub type Utf8Address = u16;
pub type ClassAddress = u16;
pub type NameAndTypeAddress = u16;

pub enum Constant {
    FieldReference(ClassReference),
    MethodReference(ClassReference),
    InterfaceMethodReference(ClassReference),

    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),

    Class {
        name_index: Utf8Address,
    },

    String {
        string_index: Utf8Address,
    },

    Utf8(String),

    NameAndType {
        name_index: Utf8Address,
        descriptor_index: Utf8Address,
    },
}

impl Constant {
    /// Parse a constant given the costant tag and binary data
    pub fn from_tag(tag: u8, byte_shifter: &mut ByteShifter) -> Self {
        match tag {
            1 => {
                let length = byte_shifter.read_2_bytes();
                let mut contents = String::new();

                for _ in 0..length {
                    contents += &char::from(byte_shifter.read_byte()).to_string();
                }

                Constant::Utf8(contents)
            }

            3 => Constant::Integer(byte_shifter.read_4_bytes() as i32),
            4 => Constant::Float(byte_shifter.read_4_bytes() as f32),
            5 => Constant::Long(byte_shifter.read_8_bytes() as i64),
            6 => Constant::Double(byte_shifter.read_8_bytes() as f64),

            7 => Constant::Class {
                name_index: byte_shifter.read_2_bytes(),
            },

            8 => Constant::String {
                string_index: byte_shifter.read_2_bytes(),
            },

            9 => Constant::FieldReference(ClassReference {
                class_index: byte_shifter.read_2_bytes(),
                name_and_type_index: byte_shifter.read_2_bytes(),
            }),

            10 => Constant::MethodReference(ClassReference {
                class_index: byte_shifter.read_2_bytes(),
                name_and_type_index: byte_shifter.read_2_bytes(),
            }),

            11 => Constant::InterfaceMethodReference(ClassReference {
                class_index: byte_shifter.read_2_bytes(),
                name_and_type_index: byte_shifter.read_2_bytes(),
            }),

            12 => Constant::NameAndType {
                name_index: byte_shifter.read_2_bytes(),
                descriptor_index: byte_shifter.read_2_bytes(),
            },

            _ => panic!("tag {} not implemented", tag),
        }
    }
}

impl Debug for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Type: ")?;

        match self {
            Self::Utf8(contents) => {
                write!(f, "{}, contents: {}", "Utf8".blue(), contents.yellow())
            }

            Self::Integer(x) => {
                write!(f, "{}, value: {}", "Integer".blue(), x.to_string().yellow())
            }

            Self::Float(x) => write!(f, "{}, value: {}", "Float".blue(), x.to_string().yellow()),
            Self::Long(x) => write!(f, "{}, value: {}", "Long".blue(), x.to_string().yellow()),
            Self::Double(x) => write!(f, "{}, value: {}", "Double".blue(), x.to_string().yellow()),

            Self::Class { name_index } => {
                write!(
                    f,
                    "{}, name index: {}",
                    "Class".blue(),
                    name_index.to_string().blue()
                )
            }

            Self::String { string_index } => write!(
                f,
                "{}, string index: {}",
                "String".blue(),
                string_index.to_string().blue()
            ),

            Self::FieldReference(ClassReference {
                class_index,
                name_and_type_index,
            }) => write!(
                f,
                "{}, class index: {}, name and type index: {}",
                "Field Reference".blue(),
                class_index.to_string().blue(),
                name_and_type_index.to_string().blue()
            ),

            Self::MethodReference(ClassReference {
                class_index,
                name_and_type_index,
            }) => write!(
                f,
                "{}, class index: {}, name and type index: {}",
                "Method Reference".blue(),
                class_index.to_string().blue(),
                name_and_type_index.to_string().blue()
            ),

            Self::InterfaceMethodReference(ClassReference {
                class_index,
                name_and_type_index,
            }) => write!(
                f,
                "{}, class index: {}, name and type index: {}",
                "Interface Method Reference".blue(),
                class_index.to_string().blue(),
                name_and_type_index.to_string().blue()
            ),

            Self::NameAndType {
                name_index,
                descriptor_index,
            } => write!(
                f,
                "{}, name index: {}, descriptor index: {}",
                "Name and Type".blue(),
                name_index.to_string().blue(),
                descriptor_index.to_string().blue()
            ),
        }
    }
}

pub struct ClassReference {
    class_index: ClassAddress,
    name_and_type_index: NameAndTypeAddress,
}
