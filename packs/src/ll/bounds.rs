pub const MAX_PLUS_TINY_INT: u8 = 0x7F;
pub const MIN_MINUS_TINY_INT: i8 = -16;

pub fn is_in_plus_tiny_int_bound(i: i64) -> bool {
    i < MAX_PLUS_TINY_INT as i64 && i >= 0
}

pub fn is_in_minus_tiny_int_bound(i: i64) -> bool {
    i < 0 && i >= MIN_MINUS_TINY_INT as i64
}

pub fn is_in_i8_bound(i: i64) -> bool {
    i >= i8::min_value() as i64 && i <= i8::max_value() as i64
}

pub fn is_in_i16_bound(i: i64) -> bool {
    i >= i16::min_value() as i64 && i <= i16::max_value() as i64
}

pub fn is_in_i32_bound(i: i64) -> bool {
    i >= i32::min_value() as i64 && i <= i32::max_value() as i64
}

