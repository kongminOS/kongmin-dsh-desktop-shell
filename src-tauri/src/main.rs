// Kongmin Rein — thin desktop shell for DeepSeek Harness
// Pure shell entry point.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    kongmin_rein_lib::run()
}
