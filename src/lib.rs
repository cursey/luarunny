#![allow(dead_code)]

mod api;
mod mem;

use rlua::Lua;
use std::{env::current_dir, ffi::c_void, path::PathBuf, thread};
use windows::Win32::{
    Foundation::{BOOL, HINSTANCE},
    System::{LibraryLoader::GetModuleFileNameA, SystemServices::DLL_PROCESS_ATTACH},
};

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
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::new(filepath))),
    )
}

struct MyApp {
    filepath: PathBuf,
    name: String,
    age: u32,
    lua: Lua,
}

impl MyApp {
    pub fn new(filepath: PathBuf) -> Self {
        let lua = Lua::new();
        lua.context(|ctx| {
            api::register(&ctx).expect("Failed to register the API with the Lua context");
        });
        Self {
            filepath,
            name: "cursey".to_string(),
            age: 30,
            lua,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Applicaiton");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            ui.label(format!(
                "curdir: '{}'",
                current_dir().unwrap().to_str().unwrap()
            ));
            ui.label(format!("modpath: '{}'", self.filepath.to_str().unwrap()));
            if ui.button("run script").clicked() {
                self.lua.context(|ctx| {
                    assert_eq!(ctx.load("mem.test(2, 3)").eval::<i32>().unwrap(), 5);
                });
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
