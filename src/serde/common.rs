// Major types constants, used to avoid writing major types everywhere
pub(crate) const UNSIGNED_INTEGER: u8   = 0b000_00000;
pub(crate) const NEGATIVE_INTEGER: u8   = 0b001_00000;
pub(crate) const BYTE_STRING: u8        = 0b010_00000;
pub(crate) const TEXT_STRING: u8        = 0b011_00000;
pub(crate) const ARRAY_OF_ITEMS: u8     = 0b100_00000;
pub(crate) const MAP_OF_ITEMS: u8       = 0b101_00000;
pub(crate) const TAGGED_ITEM: u8        = 0b110_00000;
pub(crate) const FLOAT_SIMPLE_BREAK: u8 = 0b111_00000;
