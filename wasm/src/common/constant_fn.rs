pub const fn parse_version_u32(v: &str) -> u32 {
    let bytes = v.as_bytes();
    let mut i = 0;
    let mut major = 0u32;
    let mut minor = 0u32;
    let mut patch = 0u32;

    // major
    while i < bytes.len() && bytes[i] != b'.' {
        major = major * 10 + (bytes[i] - b'0') as u32;
        i += 1;
    }
    i += 1;

    // minor
    while i < bytes.len() && bytes[i] != b'.' {
        minor = minor * 10 + (bytes[i] - b'0') as u32;
        i += 1;
    }
    i += 1;

    // patch
    while i < bytes.len() {
        patch = patch * 10 + (bytes[i] - b'0') as u32;
        i += 1;
    }

    (major << 16) | (minor << 8) | patch
}
