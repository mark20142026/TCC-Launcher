//! TCC Launcher entry point

fn main() {
    tcc_app::run(cfg!(debug_assertions));
}
