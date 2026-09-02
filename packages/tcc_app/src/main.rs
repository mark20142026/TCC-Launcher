// Hide the console window in distributed (release) builds; debug builds keep
// it so development logging stays visible.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! TCC Launcher entry point

fn main() {
    tcc_app::run(cfg!(debug_assertions));
}
