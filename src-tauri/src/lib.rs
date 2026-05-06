//! AetherMail - A modern email client

mod pop3;
mod imap;
mod smtp;
mod storage;
mod commands;
mod ai;
mod utils;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use commands::email::{init_database, cleanup_expired_trash};

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
        .plugin(tauri_plugin_log::Builder::new()
            .targets([
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
            ])
            .level(log::LevelFilter::Info)
            // 过滤掉 async-openai 库的内部日志
            .filter(|metadata| {
                // 只显示我们自己的日志，过滤掉第三方库的 verbose 日志
                let target = metadata.target();
                // 保留 aethermail 自己的模块日志
                if target.starts_with("aethermail") {
                    true
                } else if target.contains("async_openai") || target.contains("reqwest") {
                    // 过滤掉 async-openai 和 reqwest 的内部日志
                    metadata.level() <= log::LevelFilter::Warn
                } else {
                    // 其他库的日志，只显示 warn 及以上
                    metadata.level() <= log::LevelFilter::Warn
                }
            })
            .build())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Account commands
            commands::test_pop3_connection,
            commands::test_imap_connection,
            commands::test_smtp_connection,
            commands::add_pop3_account,
            commands::add_imap_account,
            commands::get_accounts,
            commands::get_active_account,
            commands::set_active_account,
            commands::update_account,
            commands::delete_account,
            commands::update_account_phishing_detection,
            commands::update_account_signature,
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
            commands::delete_email,
            commands::get_unread_count,
            commands::send_email,
            // Phishing commands
            commands::update_phishing_status,
            commands::get_phishing_emails,
            commands::get_phishing_count,
            commands::get_phishing_unread_count,
            // Starred commands
            commands::get_starred_emails,
            commands::get_starred_count,
            // Sent/Draft commands (已发送/草稿)
            commands::get_sent_emails,
            commands::get_sent_count,
            commands::get_draft_emails,
            commands::get_draft_count,
            commands::create_draft,
            commands::update_draft,
            commands::delete_draft,
            commands::send_draft,
            commands::get_draft_detail,
            commands::get_draft_attachments,
            // Attachment file operations
            commands::read_file_for_attachment,
            commands::validate_attachment_path,
            // Debug commands
            commands::debug_list_all_draft_attachments,
            commands::debug_list_draft_email_ids,
            // Database readiness check
            commands::is_database_ready,
            commands::wait_for_database,
            // Trash commands (回收站)
            commands::move_to_trash,
            commands::get_trash_emails,
            commands::get_trash_count,
            commands::restore_email,
            commands::permanent_delete,
            commands::cleanup_expired_trash,
            // Attachment commands (懒加载)
            commands::download_attachment,
            commands::save_attachment_to_path,
            commands::get_attachment_info,
            commands::get_attachments_with_status,
            commands::get_attachment_cache_stats,
            commands::clear_attachment_cache,
            commands::open_attachment_cache_directory,
            // Contact commands (通讯录)
            commands::search_contacts,
            commands::get_all_contacts,
            commands::get_contact_by_email,
            commands::update_contact,
            commands::delete_contact,
            // Thread commands (邮件会话)
            commands::get_threads,
            commands::get_thread_emails,
            commands::assign_threads,
            commands::get_emails_for_threading,
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

                // 清理过期回收站邮件（30天前删除的）
                if let Ok(cleaned) = cleanup_expired_trash().await {
                    if cleaned > 0 {
                        log::info!("自动清理了 {} 封过期回收站邮件", cleaned);
                    }
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