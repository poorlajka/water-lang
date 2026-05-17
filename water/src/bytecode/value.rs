pub type Value = u64;

pub const TAG_MASK:    Value = 0b111;
pub const TAG_INT:     Value = 0b001;
pub const TAG_POINTER: Value = 0b000;
pub const TAG_BOOL:    Value = 0b011;

pub fn tag_int(n: i64) -> Value {
    ((n << 3) as u64) | TAG_INT
}

pub fn untag_int(val: Value) -> i64 {
    (val as i64) >> 3
}

pub fn is_int(val: Value) -> bool {
    val & TAG_MASK == TAG_INT
}

pub fn tag_pointer(ptr: usize) -> Value {
    ptr as Value
}

pub fn untag_pointer(val: Value) -> usize {
    val as usize
}

pub fn is_pointer(val: Value) -> bool {
    val & TAG_MASK == TAG_POINTER
}

pub fn tag_bool(b: bool) -> Value {
    ((b as u64) << 3) | TAG_BOOL
}

pub fn untag_bool(val: Value) -> bool {
    (val >> 3) & 1 == 1
}

pub fn is_bool(val: Value) -> bool {
    val & TAG_MASK == TAG_BOOL
}