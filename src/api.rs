use crate::mem::*;
use anyhow::Result;
use rlua::Context;

pub fn register(ctx: &Context) -> Result<()> {
    let mem = ctx.create_table()?;

    mem.set(
        "abs",
        ctx.create_function(|_ctx, address: usize| Ok(abs(address)))?,
    )?;

    mem.set(
        "xref",
        ctx.create_function(|_ctx, (start, len, address): (usize, usize, usize)| {
            Ok(xref(span(start, len), address))
        })?,
    )?;

    mem.set(
        "xrefs",
        ctx.create_function(|_ctx, (start, len, address): (usize, usize, usize)| {
            Ok(xrefs(span(start, len), address))
        })?,
    )?;

    mem.set(
        "scan",
        ctx.create_function(|_ctx, (start, len, pat): (usize, usize, String)| {
            match scan(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(rlua::Error::external(e)),
            }
        })?,
    )?;

    // rscan
    mem.set(
        "rscan",
        ctx.create_function(|_ctx, (start, len, pat): (usize, usize, String)| {
            match rscan(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(rlua::Error::external(e)),
            }
        })?,
    )?;

    mem.set(
        "scan_all",
        ctx.create_function(|_ctx, (start, len, pat): (usize, usize, String)| {
            match scan_all(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(rlua::Error::external(e)),
            }
        })?,
    )?;

    mem.set(
        "scan_str",
        ctx.create_function(|_ctx, (start, len, pat): (usize, usize, String)| {
            match scan_str(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(rlua::Error::external(e)),
            }
        })?,
    )?;

    // rscan_str
    mem.set(
        "rscan_str",
        ctx.create_function(|_ctx, (start, len, pat): (usize, usize, String)| {
            match rscan_str(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(rlua::Error::external(e)),
            }
        })?,
    )?;

    mem.set(
        "scan_all_str",
        ctx.create_function(|_ctx, (start, len, pat): (usize, usize, String)| {
            match scan_all_str(span(start, len), pat.as_str()) {
                Ok(m) => Ok(m),
                Err(e) => Err(rlua::Error::external(e)),
            }
        })?,
    )?;

    mem.set(
        "test",
        ctx.create_function(|_ctx, (a, b): (i32, i32)| Ok(a + b))?,
    )?;
    ctx.globals().set("mem", mem)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use rlua::Lua;

    #[test]
    fn mem_abs() {
        let lua = Lua::new();
        lua.context(|ctx| {
            register(&ctx).unwrap();

            let data: &[u8] = &[0x12, 0x34, 0x56, 0x78];
            let a = data.as_ptr() as usize;
            let script = format!("return mem.abs(0x{:x})", a);

            assert_eq!(
                ctx.load(script.as_str()).eval::<usize>().unwrap(),
                a + 0x78563416
            );
        });
    }

    #[test]
    fn mem_xref() {
        let lua = Lua::new();
        lua.context(|ctx| {
            register(&ctx).unwrap();

            let data: &[u8] = &[1, 2, 3, 4, 0x12, 0x34, 0x56, 0x78];
            let a = data.as_ptr() as usize;
            let script = format!(
                "return mem.xref(0x{:x}, {}, 0x{:x} + 4 + 0x78563416)",
                a,
                data.len(),
                a,
            );

            assert_eq!(
                ctx.load(script.as_str()).eval::<Option<usize>>().unwrap(),
                Some(a + 4)
            );
        });
    }

    #[test]
    fn mem_xrefs() {
        let lua = Lua::new();
        lua.context(|ctx| {
            register(&ctx).unwrap();

            let data: &[u8] = &[1, 2, 3, 4, 0x12, 0x34, 0x56, 0x78];
            let a = data.as_ptr() as usize;
            let script = format!(
                "return mem.xrefs(0x{:x}, {}, 0x{:x} + 4 + 0x78563416)",
                a,
                data.len(),
                a,
            );

            assert_eq!(
                ctx.load(script.as_str()).eval::<Vec<usize>>().unwrap(),
                vec![a + 4]
            );
        });
    }

    #[test]
    fn mem_scan() {
        let lua = Lua::new();
        lua.context(|ctx| {
            register(&ctx).unwrap();

            let data: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf];
            let script = format!(
                r#"return mem.scan(0x{:x}, {}, "0a ? 0C")"#,
                data.as_ptr() as usize,
                data.len()
            );

            assert_eq!(
                ctx.load(script.as_str()).eval::<Option<usize>>().unwrap(),
                Some(data.as_ptr() as usize + 10)
            );
        });
    }

    #[test]
    fn mem_scan_all() {
        let lua = Lua::new();
        lua.context(|ctx| {
            register(&ctx).unwrap();

            let data: &[u8] = &[0, 1, 2, 3, 42, 77, 0, 1, 2, 3, 5, 6, 7];
            let script = format!(
                r#"return mem.scan_all(0x{:x}, {}, "00 ? ? 03")"#,
                data.as_ptr() as usize,
                data.len()
            );

            assert_eq!(
                ctx.load(script.as_str()).eval::<Vec<usize>>().unwrap(),
                vec![data.as_ptr() as usize + 0, data.as_ptr() as usize + 6]
            );
        });
    }

    #[test]
    fn mem_scan_str() {
        let lua = Lua::new();
        lua.context(|ctx| {
            register(&ctx).unwrap();

            let data = "Hello, world!";
            let script = format!(
                r#"return mem.scan_str(0x{:x}, {}, "world")"#,
                data.as_ptr() as usize,
                data.len()
            );

            assert_eq!(
                ctx.load(script.as_str()).eval::<Option<usize>>().unwrap(),
                Some(data.as_ptr() as usize + 7)
            );
        });
    }

    #[test]
    fn mem_scan_all_str() {
        let lua = Lua::new();
        lua.context(|ctx| {
            register(&ctx).unwrap();

            let data = "Hello, world! Hello, moon!";
            let script = format!(
                r#"return mem.scan_all_str(0x{:x}, {}, "Hello")"#,
                data.as_ptr() as usize,
                data.len()
            );

            assert_eq!(
                ctx.load(script.as_str()).eval::<Vec<usize>>().unwrap(),
                vec![data.as_ptr() as usize + 0, data.as_ptr() as usize + 14]
            );
        });
    }
}
