//! AetherMail - A modern email client

mod pop3;
mod storage;
mod commands;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use commands::email::init_database;

pub fn run() {
    // Windows 平台设置 WebView2 环境变量，降低 GPU 占用
    #[cfg(target_os = "windows")]
    {
        // 禁用 GPU 硬件加速相关参数
        std::env::set_var(
            "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
            "--disable-gpu --disable-gpu-compositing --disable-software-rasterizer --disable-background-timer-throttling --disable-renderer-backgrounding"
        );
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Account commands
            commands::test_pop3_connection,
            commands::add_pop3_account,
            commands::get_accounts,
            commands::get_active_account,
            commands::set_active_account,
            commands::update_account,
            commands::delete_account,
            commands::update_account_phishing_detection,
            // AI commands
            commands::get_ai_config,
            commands::save_ai_config,
            commands::update_ai_config,
            commands::get_config_dir,
            commands::is_ai_available,
            commands::load_phishing_prompt,
            commands::reset_phishing_prompt,
            commands::get_phishing_prompt_path_str,
            // Email commands
            commands::sync_emails,
            commands::get_emails,
            commands::get_email_detail,
            commands::mark_email_read,
            commands::toggle_email_star,
            commands::delete_account_emails,
            commands::get_unread_count,
            // Phishing commands
            commands::update_phishing_status,
            commands::get_phishing_emails,
            commands::get_phishing_count,
            commands::get_phishing_unread_count,
        ])
        .setup(|app| {
            // Initialize database
            let _app_handle = app.handle().clone();
            let app_data_dir = app.path().app_data_dir()
                .expect("无法获取应用数据目录");

            tauri::async_runtime::spawn(async move {
                if let Err(e) = init_database(app_data_dir).await {
                    log::error!("初始化数据库失败: {}", e);
                }
            });

            // Build system tray menu
            let show = MenuItem::with_id(app, "show", "显示 AetherMail", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            let icon = app
                .default_window_icon()
                .cloned()
                .expect("应用应该配置了默认图标");

            TrayIconBuilder::with_id("main-tray")
                .icon(icon)
                .tooltip("AetherMail")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // Minimize to tray on close
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    let _ = window.hide();
                    api.prevent_close();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用时出错");
}