use std::fmt::Debug;

use colored::Colorize;

/// A mask of flags to denote access permissions and properties of a class or interface
pub struct AccessFlags(u16);

/// Simple macro to make functions out of bitmasks
#[macro_export]
macro_rules! bitmask {
    ($visibility:vis, $function_name:ident, $bitmask:expr) => {
        $visibility fn $function_name(&self) -> bool {
            (self.0 & $bitmask) == $bitmask
        }
    };
}

impl AccessFlags {
    pub fn new(flags: u16) -> Self {
        Self(flags)
    }

    bitmask! {pub, is_public,     0x0001}
    bitmask! {pub, is_final,      0x0010}
    bitmask! {pub, is_super,      0x0020}
    bitmask! {pub, is_interface,  0x0200}
    bitmask! {pub, is_abstract,   0x0400}
    bitmask! {pub, is_synthethic, 0x1000}
    bitmask! {pub, is_annotation, 0x2000}
    bitmask! {pub, is_enum,       0x4000}
    bitmask! {pub, is_module,     0x8000}
}

impl Debug for AccessFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "Public: {}, Final: {}, Super: {}, Interface: {}, Abstract: {}, Synthetic: {}, Annotation: {}, Enum: {}, Module: {}",
            if self.is_public() { "✔ ️".green() } else { "✖ ".red() },
            if self.is_final() { "✔ ️".green() } else { "✖ ".red() },
            if self.is_super() { "✔ ️".green() } else { "✖ ".red() },
            if self.is_interface() { "✔ ️".green() } else { "✖ ".red() },
            if self.is_abstract() { "✔ ️".green() } else { "✖ ".red() },
            if self.is_synthethic() { "✔ ️".green() } else { "✖ ".red() },
            if self.is_annotation() { "✔ ️".green() } else { "✖ ".red() },
            if self.is_enum() { "✔ ️".green() } else { "✖ ".red() },
            if self.is_module() { "✔ ️".green() } else { "✖ ".red() }
        )
    }
}
