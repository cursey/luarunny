use crate::mem::*;
use mlua::prelude::*;

pub fn register(lua: &Lua) -> LuaResult<()> {
    let mem = lua.create_table()?;

    mem.set(
        "abs",
        lua.create_function(|_, address: usize| Ok(abs(address)))?,
    )?;

    mem.set(
        "xref",
        lua.create_function(|_, (start, len, address): (usize, usize, usize)| {
            Ok(xref(span(start, len), address))
        })?,
    )?;

    mem.set(
        "xrefs",
        lua.create_function(|_, (start, len, address): (usize, usize, usize)| {
            Ok(xrefs(span(start, len), address))
        })?,
    )?;

    mem.set(
        "scan",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            match scan(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(mlua::Error::external(e)),
            }
        })?,
    )?;

    // rscan
    mem.set(
        "rscan",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            match rscan(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(mlua::Error::external(e)),
            }
        })?,
    )?;

    mem.set(
        "scan_all",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            match scan_all(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(mlua::Error::external(e)),
            }
        })?,
    )?;

    mem.set(
        "scan_str",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            match scan_str(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(mlua::Error::external(e)),
            }
        })?,
    )?;

    // rscan_str
    mem.set(
        "rscan_str",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            match rscan_str(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(mlua::Error::external(e)),
            }
        })?,
    )?;

    mem.set(
        "scan_all_str",
        lua.create_function(|_, (start, len, pat): (usize, usize, String)| {
            match scan_all_str(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(mlua::Error::external(e)),
            }
        })?,
    )?;

    // read_i8
    mem.set(
        "read_i8",
        lua.create_function(|_, address: usize| Ok(read_i8(address)))?,
    )?;

    // read_i16
    mem.set(
        "read_i16",
        lua.create_function(|_, address: usize| Ok(read_i16(address)))?,
    )?;

    // read_i32
    mem.set(
        "read_i32",
        lua.create_function(|_, address: usize| Ok(read_i32(address)))?,
    )?;

    // read_i64
    mem.set(
        "read_i64",
        lua.create_function(|_, address: usize| Ok(read_i64(address)))?,
    )?;

    // read_f32
    mem.set(
        "read_f32",
        lua.create_function(|_, address: usize| Ok(read_f32(address)))?,
    )?;

    // read_f64
    mem.set(
        "read_f64",
        lua.create_function(|_, address: usize| Ok(read_f64(address)))?,
    )?;

    // read_u8
    mem.set(
        "read_u8",
        lua.create_function(|_, address: usize| Ok(read_u8(address)))?,
    )?;

    // read_u16
    mem.set(
        "read_u16",
        lua.create_function(|_, address: usize| Ok(read_u16(address)))?,
    )?;

    // read_u32
    mem.set(
        "read_u32",
        lua.create_function(|_, address: usize| Ok(read_u32(address)))?,
    )?;

    // read_u64
    mem.set(
        "read_u64",
        lua.create_function(|_, address: usize| Ok(read_u64(address)))?,
    )?;

    // write_i8
    mem.set(
        "write_i8",
        lua.create_function(|_, (address, value): (usize, i8)| Ok(write_i8(address, value)))?,
    )?;

    // write_i16
    mem.set(
        "write_i16",
        lua.create_function(|_, (address, value): (usize, i16)| Ok(write_i16(address, value)))?,
    )?;

    // write_i32
    mem.set(
        "write_i32",
        lua.create_function(|_, (address, value): (usize, i32)| Ok(write_i32(address, value)))?,
    )?;

    // write_i64
    mem.set(
        "write_i64",
        lua.create_function(|_, (address, value): (usize, i64)| Ok(write_i64(address, value)))?,
    )?;

    // write_f32
    mem.set(
        "write_f32",
        lua.create_function(|_, (address, value): (usize, f32)| Ok(write_f32(address, value)))?,
    )?;

    // write_f64
    mem.set(
        "write_f64",
        lua.create_function(|_, (address, value): (usize, f64)| Ok(write_f64(address, value)))?,
    )?;

    // write_u8
    mem.set(
        "write_u8",
        lua.create_function(|_, (address, value): (usize, u8)| Ok(write_u8(address, value)))?,
    )?;

    // write_u16
    mem.set(
        "write_u16",
        lua.create_function(|_, (address, value): (usize, u16)| Ok(write_u16(address, value)))?,
    )?;

    // write_u32
    mem.set(
        "write_u32",
        lua.create_function(|_, (address, value): (usize, u32)| Ok(write_u32(address, value)))?,
    )?;

    // write_u64
    mem.set(
        "write_u64",
        lua.create_function(|_, (address, value): (usize, u64)| Ok(write_u64(address, value)))?,
    )?;

    lua.globals().set("mem", mem)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use mlua::Lua;

    #[test]
    fn mem_abs() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data: &[u8] = &[0x12, 0x34, 0x56, 0x78];
        let a = data.as_ptr() as usize;
        let script = format!("return mem.abs(0x{:x})", a);

        assert_eq!(
            lua.load(script.as_str()).eval::<usize>().unwrap(),
            a + 0x78563416
        );
    }

    #[test]
    fn mem_xref() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data: &[u8] = &[1, 2, 3, 4, 0x12, 0x34, 0x56, 0x78];
        let a = data.as_ptr() as usize;
        let script = format!(
            "return mem.xref(0x{:x}, {}, 0x{:x} + 4 + 0x78563416)",
            a,
            data.len(),
            a,
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Option<usize>>().unwrap(),
            Some(a + 4)
        );
    }

    #[test]
    fn mem_xrefs() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data: &[u8] = &[1, 2, 3, 4, 0x12, 0x34, 0x56, 0x78];
        let a = data.as_ptr() as usize;
        let script = format!(
            "return mem.xrefs(0x{:x}, {}, 0x{:x} + 4 + 0x78563416)",
            a,
            data.len(),
            a,
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Vec<usize>>().unwrap(),
            vec![a + 4]
        );
    }

    #[test]
    fn mem_scan() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];
        let script = format!(
            r#"return mem.scan(0x{:x}, {}, "0a ? 0C")"#,
            data.as_ptr() as usize,
            data.len()
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Option<usize>>().unwrap(),
            Some(data.as_ptr() as usize + 10)
        );
    }

    #[test]
    fn mem_scan_all() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data: &[u8] = &[0, 1, 2, 3, 42, 77, 0, 1, 2, 3, 5, 6, 7];
        let script = format!(
            r#"return mem.scan_all(0x{:x}, {}, "00 ? ? 03")"#,
            data.as_ptr() as usize,
            data.len()
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Vec<usize>>().unwrap(),
            vec![data.as_ptr() as usize + 0, data.as_ptr() as usize + 6]
        );
    }

    #[test]
    fn mem_scan_str() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data = "Hello, world!";
        let script = format!(
            r#"return mem.scan_str(0x{:x}, {}, "world")"#,
            data.as_ptr() as usize,
            data.len()
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Option<usize>>().unwrap(),
            Some(data.as_ptr() as usize + 7)
        );
    }

    #[test]
    fn mem_scan_all_str() {
        let lua = Lua::new();
        register(&lua).unwrap();

        let data = "Hello, world! Hello, moon!";
        let script = format!(
            r#"return mem.scan_all_str(0x{:x}, {}, "Hello")"#,
            data.as_ptr() as usize,
            data.len()
        );

        assert_eq!(
            lua.load(script.as_str()).eval::<Vec<usize>>().unwrap(),
            vec![data.as_ptr() as usize + 0, data.as_ptr() as usize + 14]
        );
    }
}
