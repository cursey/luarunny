#![allow(dead_code)]

use mlua::prelude::*;
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

lazy_static! {
    static ref MSG: Mutex<String> = Mutex::new(String::new());
    static ref WANTS_EXIT: Mutex<bool> = Mutex::new(false);
    static ref WANTS_RESET: Mutex<bool> = Mutex::new(false);
    static ref WANTS_ALWAYS_ON_TOP: Mutex<bool> = Mutex::new(false);
}

fn start_thread(module: HINSTANCE) {
    let mut filename = vec![0u8; 1024];
    let len = unsafe { GetModuleFileNameA(module, &mut filename) };
    filename.resize(len as usize, 0);

    while !*WANTS_EXIT.lock().unwrap() || *WANTS_RESET.lock().unwrap() {
        MSG.lock().unwrap().clear();
        *WANTS_EXIT.lock().unwrap() = false;
        *WANTS_RESET.lock().unwrap() = false;

        let mut filepath = PathBuf::from(String::from_utf8(filename.clone()).unwrap());
        filepath.pop();
        let mut options = eframe::NativeOptions::default();

        options.always_on_top = *WANTS_ALWAYS_ON_TOP.lock().unwrap();

        eframe::run_native(
            "LuaRunny",
            options,
            Box::new(|_cc| Box::new(LuaRunny::new(filepath))),
        )
    }
}

fn print_msg(msg: &str) {
    MSG.lock().unwrap().push_str(msg);
}

struct LuaRunny {
    filepath: PathBuf,
    lua: Lua,
    input: String,
    line: String,
}

impl LuaRunny {
    pub fn new(filepath: PathBuf) -> Self {
        let mut me = Self {
            filepath,
            lua: Lua::new(),
            input: "".to_string(),
            line: "".to_string(),
        };

        me.reset();

        me
    }

    fn reset(&mut self) {
        let lua = unsafe { Lua::unsafe_new() };

        lua.set_named_registry_value("wants_reset", false).unwrap();

        //api::register(&lua).unwrap();

        lua.globals()
            .set(
                "print",
                lua.create_function(|_, mut msg: String| {
                    msg.push_str("\n");
                    print_msg(&msg);
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "cls",
                lua.create_function(|_, _: ()| {
                    MSG.lock().unwrap().clear();
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "reset",
                lua.create_function_mut(|lua, _: ()| {
                    lua.set_named_registry_value("wants_reset", true)?;
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        lua.globals()
            .set(
                "always_on_top",
                lua.create_function_mut(|lua, always_on_top: bool| {
                    *WANTS_ALWAYS_ON_TOP.lock().unwrap() = always_on_top;
                    lua.set_named_registry_value("wants_reset", true)?;
                    Ok(())
                })
                .unwrap(),
            )
            .unwrap();

        self.lua = lua;

        print_msg("Welcome to LuaRunny!\n");
    }

    fn on_input(&mut self, frame: &mut eframe::Frame) {
        if !self.line.is_empty() {
            print_msg(format!(">> {}\n", self.input).as_str());
        } else if MSG.lock().unwrap().is_empty() {
            print_msg(format!("> {}\n", self.input).as_str());
        } else {
            print_msg(format!("\n> {}\n", self.input).as_str());
        }

        self.line.push_str(&self.input);

        match self.lua.load(&self.line).eval::<mlua::MultiValue>() {
            Ok(values) => {
                if values.len() > 0 {
                    print_msg(
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
                self.line.clear();
            }
            Err(mlua::Error::SyntaxError {
                incomplete_input: true,
                ..
            }) => {
                self.line.push_str("\n");
            }
            Err(e) => {
                print_msg(format!("{}\n", e).as_str());
                self.line.clear()
            }
        }

        self.input.clear();

        if self.lua.named_registry_value("wants_reset").unwrap() {
            *WANTS_RESET.lock().unwrap() = true;
            frame.close();
        }
    }
}

impl eframe::App for LuaRunny {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    egui::TextEdit::multiline(&mut *MSG.lock().unwrap())
                        .interactive(false)
                        .cursor_at_end(true)
                        .desired_width(f32::INFINITY)
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
                        self.on_input(frame);
                    }
                });
        });
    }

    fn on_close_event(&mut self) -> bool {
        *WANTS_EXIT.lock().unwrap() = true;
        true
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
