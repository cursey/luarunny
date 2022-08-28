#![allow(dead_code)]

mod api;
mod mem;

use rlua::{InitFlags, Lua, MultiValue, StdLib};
use std::{ffi::c_void, path::PathBuf, sync::Mutex, thread};
use windows::Win32::{
    Foundation::{BOOL, HINSTANCE},
    System::{LibraryLoader::GetModuleFileNameA, SystemServices::DLL_PROCESS_ATTACH},
};

#[macro_use]
extern crate lazy_static;

#[no_mangle]
extern "system" fn DllMain(module: HINSTANCE, reason: u32, _reserved: *const c_void) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        thread::spawn(move || {
            start_thread(module);
        });
    }

    true.into()
}

fn start_thread(module: HINSTANCE) {
    let mut filename = vec![0u8; 1024];
    let len = unsafe { GetModuleFileNameA(module, &mut filename) };
    filename.resize(len as usize, 0);
    let mut filepath = PathBuf::from(String::from_utf8(filename).unwrap());
    filepath.pop();
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "LuaRunny",
        options,
        Box::new(|_cc| Box::new(MyApp::new(filepath))),
    )
}

lazy_static! {
    static ref MSG: Mutex<String> = Mutex::new(String::new());
}

struct MyApp {
    filepath: PathBuf,
    name: String,
    age: u32,
    lua: Lua,
    input: String,
}

impl MyApp {
    pub fn new(filepath: PathBuf) -> Self {
        let lua = unsafe {
            Lua::unsafe_new_with_flags(
                StdLib::ALL_NO_DEBUG,
                InitFlags::PCALL_WRAPPERS | InitFlags::LOAD_WRAPPERS,
            )
        };

        lua.context(|ctx| {
            api::register(&ctx).expect("Failed to register the API with the Lua context");

            ctx.globals()
                .set(
                    "print",
                    ctx.create_function(|_ctx, str: String| {
                        MSG.lock().unwrap().push_str(&str);
                        Ok(())
                    })
                    .expect("Failed to create the print function"),
                )
                .expect("Failed to register the print function");
        });

        Self {
            filepath,
            name: "cursey".to_string(),
            age: 30,
            lua,
            input: "".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TextEdit::multiline(&mut *MSG.lock().unwrap())
                .interactive(false)
                .cursor_at_end(true)
                .desired_width(f32::INFINITY)
                .desired_rows(25)
                .font(egui::TextStyle::Monospace)
                .show(ui);

            let output = egui::TextEdit::singleline(&mut self.input)
                .interactive(true)
                .desired_width(f32::INFINITY)
                .font(egui::TextStyle::Monospace)
                .lock_focus(true)
                .show(ui);

            if output.response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                // refocus the input field
                output.response.request_focus();

                MSG.lock()
                    .unwrap()
                    .push_str(format!("> {}\n", self.input).as_str());
                self.lua
                    .context(|ctx| match ctx.load(&self.input).eval::<MultiValue>() {
                        Ok(values) => {
                            MSG.lock().unwrap().push_str(
                                format!(
                                    "{}\n",
                                    values
                                        .iter()
                                        .map(|value| format!("{:?}", value))
                                        .collect::<Vec<_>>()
                                        .join("\t")
                                )
                                .as_str(),
                            );
                        }
                        Err(e) => {
                            MSG.lock().unwrap().push_str(format!("{}\n", e).as_str());
                        }
                    });
                self.input.clear();
            }
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn do_thing() {
        start_thread(HINSTANCE(0));
    }
}
