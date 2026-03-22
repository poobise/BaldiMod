#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

mod lucina;
mod custom;

pub fn check_deps() -> bool {
    let mut passed = true;

    for dep in [
        "rom:/skyline/plugins/libparam_config.nro",
        "rom:/skyline/plugins/libthe_csk_collection.nro",
        "rom:/skyline/plugins/libarcropolis.nro",
        "rom:/skyline/plugins/libnro_hook.nro",
        "rom:/skyline/plugins/libsmashline_plugin.nro",
    ] {
        if !std::path::Path::new(dep).is_file() {
            println!("{} not found! This installation is incomplete. Please download all dependencies listed in the README file.", dep);
            passed = false;
        }
    }

    passed
}

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    // EVERYTHING HERE IS UNIQUE THAT'S MEANT TO BE MODIFIED FOR YOUR SPECIFIC PLUGIN.

    const FIGHTER_NAME: &str = "lucina";
    const MARKER_FILE: &str = "baldi.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/model/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }


    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }

    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            index - lowest_color
        }
    };

    // Preform csk collection calls here (allow online, narration characall entry, add chara db, add layout db, w/e)
}

#[skyline::main(name = "smashline_test")]
pub fn main() {
    if !check_deps() {
        return; // don't do anything else since they don't have all dependencies
    }

    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }

    lucina::install();
}