pub fn into_u32_le(s: &[u8]) -> u32 {
    let mut bytes: [u8; 4] = Default::default();
    bytes.copy_from_slice(s);
    let value = u32::from_le_bytes(bytes);
    value
}

pub fn into_u64_le(s: &[u8]) -> u64 {
    let mut bytes: [u8; 8] = Default::default();
    bytes.copy_from_slice(s);
    let value = u64::from_le_bytes(bytes);
    value
}
