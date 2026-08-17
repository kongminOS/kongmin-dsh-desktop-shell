// Kongmin Rein — thin desktop shell for DeepSeek Harness
// Pure shell library: opens the official DSH web UI (http://127.0.0.1:3080)
// in a native window. No governance, no plugins, no services.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::TcpStream;
use std::time::Duration;

/// 探测引擎是否就绪（最多等 5 秒）
fn engine_ready() -> bool {
    for _ in 0..5 {
        if TcpStream::connect_timeout(&"127.0.0.1:3080".parse().unwrap(), Duration::from_millis(500))
            .is_ok()
        {
            return true;
        }
        std::thread::sleep(Duration::from_millis(1000));
    }
    false
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            if !engine_ready() {
                println!("[rein] DSH engine not detected on 3080. Run `dsh web` first.");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
