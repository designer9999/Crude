mod commands;
mod lan;

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    Manager,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    image::Image,
};

/// When true, the app is actually quitting — don't intercept CloseRequested.
static QUITTING: AtomicBool = AtomicBool::new(false);


pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Store app handle for LAN events
            let handle = app.handle().clone();
            app.manage(lan::LanState::new(handle));

            // ── System Tray ──
            let show = MenuItemBuilder::with_id("show", "Show LanDrop").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
            let menu = MenuBuilder::new(app)
                .item(&show)
                .separator()
                .item(&quit)
                .build()?;

            let icon = app.default_window_icon().cloned()
                .unwrap_or_else(|| Image::from_path("icons/32x32.png").expect("tray icon"));

            let _tray = TrayIconBuilder::with_id("main-tray")
                .icon(icon)
                .tooltip("LanDrop")
                .menu(&menu)
                .on_menu_event(|app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(win) = app.get_webview_window("main") {
                                let _ = win.show();
                                let _ = win.unminimize();
                                let _ = win.set_focus();
                            }
                        }
                        "quit" => {
                            // Remove tray icon — set_visible(false) calls Shell_NotifyIcon(NIM_DELETE)
                            if let Some(tray) = app.tray_by_id("main-tray") {
                                let _ = tray.set_visible(false);
                            }
                            // Give Windows time to process the icon removal, then hard exit
                            std::thread::sleep(std::time::Duration::from_millis(150));
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::DoubleClick { .. } = event {
                        if let Some(win) = tray.app_handle().get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.unminimize();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(app)?;

            // ── Minimize to tray on close (unless actually quitting) ──
            if let Some(win) = app.get_webview_window("main") {
                win.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        if QUITTING.load(Ordering::SeqCst) {
                            // Let the close happen — app is exiting
                            return;
                        }
                        api.prevent_close();
                        // Hide window instead of closing — tray keeps app alive
                        if let Some(w) = _tray.app_handle().get_webview_window("main") {
                            let _ = w.hide();
                        }
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_status,
            commands::start_lan,
            commands::set_out_folder,
            commands::stop_lan,
            commands::lan_send_text,
            commands::lan_send_files,
            commands::get_file_info,
            commands::show_in_explorer,
            commands::get_thumbnail,
            commands::get_local_ip,
            commands::save_clipboard_image,
            commands::get_clipboard_files,
            commands::read_file_preview,
            commands::set_mica,
            commands::get_explorer_selection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running LanDrop");
}
