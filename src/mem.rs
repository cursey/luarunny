use anyhow::Result;
use std::{mem::size_of, slice};
use windows::Win32::System::{
    LibraryLoader::GetModuleHandleA,
    ProcessStatus::{K32GetModuleInformation, MODULEINFO},
    Threading::GetCurrentProcess,
};

pub fn span<'a>(start: usize, len: usize) -> &'a [u8] {
    unsafe { slice::from_raw_parts(start as *const u8, len) }
}

pub fn span_from<'a>(start: usize, end: usize) -> &'a [u8] {
    unsafe { slice::from_raw_parts(start as *const u8, end - start) }
}

pub fn abs(address: usize) -> usize {
    let offset = unsafe { *(address as *const i32) };
    address.wrapping_add(offset as usize + 4)
}

pub fn xref(mem: &[u8], address: usize) -> Option<usize> {
    let start = mem.as_ptr() as usize;
    let mut i = 0;

    // We subtract 4 from the len here because the `abs` function deref's an i32 (4 bytes).
    while i <= mem.len() - 4 {
        let possible_ref = start + i;

        if abs(possible_ref) == address {
            return Some(possible_ref);
        }

        i += 1;
    }

    return None;
}

pub fn xrefs(mem: &[u8], address: usize) -> Vec<usize> {
    let mut refs = Vec::new();
    let start = mem.as_ptr() as usize;
    let mut i = 0;

    while i <= mem.len() - 4 {
        match xref(&mem[i..], address) {
            Some(r) => {
                refs.push(r);
                i = r - start + 1;
            }
            None => break,
        }
    }

    refs
}

pub struct Pattern(Vec<Option<u8>>);

impl Pattern {
    pub fn new(pat: &str) -> Result<Self> {
        let mut mask = Vec::new();

        for b in pat.split_ascii_whitespace() {
            if b == "?" || b == "??" {
                mask.push(None);
            } else {
                mask.push(Some(u8::from_str_radix(b, 16)?));
            }
        }

        Ok(Self(mask))
    }

    pub fn from_str(pat: &str) -> Result<Self> {
        let mut mask = Vec::new();

        for c in pat.bytes() {
            mask.push(Some(c));
        }

        Ok(Self(mask))
    }

    pub fn matches(&self, mem: &[u8]) -> bool {
        if self.0.len() > mem.len() {
            return false;
        }

        let mut i = 0;

        while i < self.0.len() {
            match self.0[i] {
                Some(b) => {
                    if mem[i] != b {
                        break;
                    }
                }
                None => {}
            }

            i += 1;
        }

        i == self.0.len()
    }
}

impl TryFrom<&str> for Pattern {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

pub fn scan_pattern(mem: &[u8], pat: &Pattern) -> Option<usize> {
    let mut i = 0;

    while i < mem.len() - pat.0.len() {
        if pat.matches(&mem[i..]) {
            return Some(mem.as_ptr() as usize + i);
        }

        i += 1;
    }

    None
}

pub fn rscan_pattern(mem: &[u8], pat: &Pattern) -> Option<usize> {
    let mut i = mem.len() - pat.0.len();

    while i > 0 {
        if pat.matches(&mem[i..]) {
            return Some(mem.as_ptr() as usize + i);
        }

        i -= 1;
    }

    None
}

pub fn scan_all_pattern(mem: &[u8], pat: &Pattern) -> Vec<usize> {
    let mut refs = Vec::new();
    let mut i = 0;

    while i < mem.len() - pat.0.len() {
        match scan_pattern(&mem[i..], pat) {
            Some(r) => {
                refs.push(r);
                i = r - mem.as_ptr() as usize + 1;
            }
            None => break,
        }
    }

    refs
}

pub fn scan(mem: &[u8], pat: &str) -> Result<Option<usize>> {
    Ok(scan_pattern(mem, &Pattern::new(pat)?))
}

pub fn rscan(mem: &[u8], pat: &str) -> Result<Option<usize>> {
    Ok(rscan_pattern(mem, &Pattern::new(pat)?))
}

pub fn scan_all(mem: &[u8], pat: &str) -> Result<Vec<usize>> {
    Ok(scan_all_pattern(mem, &Pattern::new(pat)?))
}

pub fn scan_str(mem: &[u8], pat: &str) -> Result<Option<usize>> {
    Ok(scan_pattern(mem, &Pattern::from_str(pat)?))
}

pub fn rscan_str(mem: &[u8], pat: &str) -> Result<Option<usize>> {
    Ok(rscan_pattern(mem, &Pattern::from_str(pat)?))
}

pub fn scan_all_str(mem: &[u8], pat: &str) -> Result<Vec<usize>> {
    Ok(scan_all_pattern(mem, &Pattern::from_str(pat)?))
}

pub fn read_i8(address: usize) -> i8 {
    unsafe { *(address as *const i8) }
}

pub fn read_i16(address: usize) -> i16 {
    unsafe { *(address as *const i16) }
}

pub fn read_i32(address: usize) -> i32 {
    unsafe { *(address as *const i32) }
}

pub fn read_i64(address: usize) -> i64 {
    unsafe { *(address as *const i64) }
}

pub fn read_f32(address: usize) -> f32 {
    unsafe { *(address as *const f32) }
}

pub fn read_f64(address: usize) -> f64 {
    unsafe { *(address as *const f64) }
}

pub fn read_u8(address: usize) -> u8 {
    unsafe { *(address as *const u8) }
}

pub fn read_u16(address: usize) -> u16 {
    unsafe { *(address as *const u16) }
}

pub fn read_u32(address: usize) -> u32 {
    unsafe { *(address as *const u32) }
}

pub fn read_u64(address: usize) -> u64 {
    unsafe { *(address as *const u64) }
}

pub fn write_i8(address: usize, value: i8) {
    unsafe { *(address as *mut i8) = value }
}

pub fn write_i16(address: usize, value: i16) {
    unsafe { *(address as *mut i16) = value }
}

pub fn write_i32(address: usize, value: i32) {
    unsafe { *(address as *mut i32) = value }
}

pub fn write_i64(address: usize, value: i64) {
    unsafe { *(address as *mut i64) = value }
}

pub fn write_f32(address: usize, value: f32) {
    unsafe { *(address as *mut f32) = value }
}

pub fn write_f64(address: usize, value: f64) {
    unsafe { *(address as *mut f64) = value }
}

pub fn write_u8(address: usize, value: u8) {
    unsafe { *(address as *mut u8) = value }
}

pub fn write_u16(address: usize, value: u16) {
    unsafe { *(address as *mut u16) = value }
}

pub fn write_u32(address: usize, value: u32) {
    unsafe { *(address as *mut u32) = value }
}

pub fn write_u64(address: usize, value: u64) {
    unsafe { *(address as *mut u64) = value }
}

pub fn module(name: &str) -> Option<usize> {
    match unsafe { GetModuleHandleA(name) } {
        Ok(m) => Some(m.0 as usize),
        Err(_) => None,
    }
}

pub fn module_span<'a>(name: &str) -> Option<&'a [u8]> {
    match unsafe { GetModuleHandleA(name) } {
        Ok(m) => {
            let mut info = MODULEINFO::default();

            if unsafe {
                K32GetModuleInformation(
                    GetCurrentProcess(),
                    m,
                    &mut info,
                    size_of::<MODULEINFO>() as u32,
                )
            }
            .as_bool()
                == false
            {
                return None;
            }

            Some(span(info.lpBaseOfDll as usize, info.SizeOfImage as usize))
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mem_span() {
        let str = "Hello, world!";
        let mem = span(str.as_ptr() as usize, str.len());

        assert_eq!(mem.as_ptr(), str.as_ptr());
        assert_eq!(mem.len(), str.len());
    }
    #[test]
    fn mem_span_from() {
        let str = "Hello, world!";
        let mem = span_from(str.as_ptr() as usize, str.as_ptr() as usize + str.len());

        assert_eq!(mem.as_ptr(), str.as_ptr());
        assert_eq!(mem.len(), str.len());
    }
    #[test]
    fn mem_abs() {
        use std::ptr::addr_of;

        let data: &[u8] = &[0x12, 0x34, 0x56, 0x78];
        let a = data.as_ptr() as usize;

        assert_eq!(abs(a), a + 0x78563416);

        let data = -42;
        let a = addr_of!(data) as usize;

        assert_eq!(abs(a), a - 42 + 4);
    }

    #[test]
    fn mem_xref() {
        let data: &[u8] = &[1, 2, 3, 4, 0x12, 0x34, 0x56, 0x78];
        let a = data.as_ptr() as usize;

        assert_eq!(xref(data, a + 4 + 0x78563416), Some(a + 4));
    }

    #[test]
    fn mem_xrefs() {
        let data: &[u8] = &[1, 2, 3, 4, 0x12, 0x34, 0x56, 0x78];
        let a = data.as_ptr() as usize;
        let refs = xrefs(data, a + 4 + 0x78563416);

        assert_eq!(refs.len(), 1);
    }

    #[test]
    fn mem_pattern_new() {
        let p = Pattern::new("01 23 45 67 89 AB CD EF ?").unwrap();

        assert_eq!(p.0[0], Some(0x01));
        assert_eq!(p.0[1], Some(0x23));
        assert_eq!(p.0[7], Some(0xEF));
        assert_eq!(p.0[8], None);
    }

    #[test]
    fn mem_pattern_from_str() {
        let p = Pattern::from_str("Hello, world!").unwrap();

        assert_eq!(p.0[0], Some(b'H'));
        assert_eq!(p.0[1], Some(b'e'));
        assert_eq!(p.0[2], Some(b'l'));
        assert_eq!(p.0[12], Some(b'!'));
    }

    #[test]
    fn mem_pattern_try_into() {
        let p: Pattern = "Hello, world!".try_into().unwrap();

        assert_eq!(p.0[0], Some(b'H'));
        assert_eq!(p.0[1], Some(b'e'));
        assert_eq!(p.0[2], Some(b'l'));
        assert_eq!(p.0[12], Some(b'!'));
    }

    #[test]
    fn mem_pattern_try_from() {
        let p = Pattern::try_from("Hello, world!").unwrap();

        assert_eq!(p.0[0], Some(b'H'));
        assert_eq!(p.0[1], Some(b'e'));
        assert_eq!(p.0[2], Some(b'l'));
        assert_eq!(p.0[12], Some(b'!'));
    }

    #[test]
    fn mem_scan() {
        let data: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];

        assert_eq!(
            scan(data, "0a ? 0C").unwrap(),
            Some(data.as_ptr() as usize + 10)
        );
    }

    #[test]
    fn mem_scan_all() {
        let data: &[u8] = &[0, 1, 2, 3, 42, 77, 0, 1, 2, 3, 5, 6, 7];

        assert_eq!(
            scan_all(data, "00 ? ? 03").unwrap(),
            vec![data.as_ptr() as usize + 0, data.as_ptr() as usize + 6]
        );
    }

    #[test]
    fn mem_scan_str() {
        let data = "Hello, world!";

        assert_eq!(
            scan_str(data.as_bytes(), "world").unwrap(),
            Some(data.as_ptr() as usize + 7)
        );
    }

    #[test]
    fn mem_scan_all_str() {
        let data = "Hello, world! Hello, moon!";

        assert_eq!(
            scan_all_str(data.as_bytes(), "Hello").unwrap(),
            vec![data.as_ptr() as usize + 0, data.as_ptr() as usize + 14]
        );
    }

    #[test]
    fn mem_module() {
        assert!(module("kernel32.dll").is_some());
        assert!(module("nonexistent.dll").is_none());
    }

    #[test]
    fn mem_module_span() {
        assert!(module_span("kernel32.dll").is_some());
        assert!(module_span("nonexistent.dll").is_none());
    }
}
