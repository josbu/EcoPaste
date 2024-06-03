// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, generate_handler, Builder, Manager};
use tauri_plugin_theme::ThemePlugin;
mod tray;
mod window;
use window::{
    create_window, frosted_window, hide_window, quit_app, show_window, MAIN_WINDOW_LABEL,
};

fn main() {
    let mut ctx = generate_context!();

    Builder::default()
        .setup(|_app| {
            // 在开发环境中启动时打开控制台：https://tauri.app/v1/guides/debugging/application/#opening-devtools-programmatically
            #[cfg(debug_assertions)]
            {
                let window = _app.get_window(MAIN_WINDOW_LABEL).unwrap();
                window.open_devtools();
            }

            Ok(())
        })
        // 剪切板插件：https://github.com/CrossCopy/tauri-plugin-clipboard
        .plugin(tauri_plugin_clipboard::init())
        // 主题插件：https://github.com/wyhaya/tauri-plugin-theme
        .plugin(ThemePlugin::init(ctx.config_mut()))
        // 系统托盘：https://tauri.app/v1/guides/features/system-tray
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(generate_handler![
            create_window,
            hide_window,
            show_window,
            quit_app,
            frosted_window
        ])
        .run(ctx)
        .expect("error while running tauri application");
}
