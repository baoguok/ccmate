mod commands;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Configure window for macOS
            #[cfg(target_os = "macos")]
            {
                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    // Set the window to have a transparent title bar
                    let _ = window.set_title_bar_style(tauri::TitleBarStyle::Overlay);
                }
            }

            // Create application menu
            use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder, PredefinedMenuItem};

            let open_config_item = MenuItemBuilder::with_id("open_config_path", "Open config path")
                .accelerator("CmdOrCtrl+Shift+O")
                .build(app)?;

            let separator = PredefinedMenuItem::separator(app)?;

            // App menu (macOS)
            let app_name = app.package_info().name.clone();
            let app_menu = SubmenuBuilder::new(app, &app_name)
                .item(&PredefinedMenuItem::about(app, Some(&app_name), None)?)
                .item(&separator)
                .item(&PredefinedMenuItem::services(app, None)?)
                .item(&separator)
                .item(&PredefinedMenuItem::hide(app, None)?)
                .item(&PredefinedMenuItem::hide_others(app, None)?)
                .item(&PredefinedMenuItem::show_all(app, None)?)
                .item(&separator)
                .item(&PredefinedMenuItem::quit(app, Some(&app_name))?)
                .build()?;

            // File menu
            let file_menu = SubmenuBuilder::new(app, "File")
                .item(&open_config_item)
                .item(&separator)
                .item(&PredefinedMenuItem::close_window(app, None)?)
                .build()?;

            // Edit menu
            let edit_menu = SubmenuBuilder::new(app, "Edit")
                .item(&PredefinedMenuItem::undo(app, None)?)
                .item(&PredefinedMenuItem::redo(app, None)?)
                .item(&separator)
                .item(&PredefinedMenuItem::cut(app, None)?)
                .item(&PredefinedMenuItem::copy(app, None)?)
                .item(&PredefinedMenuItem::paste(app, None)?)
                .item(&separator)
                .item(&PredefinedMenuItem::select_all(app, None)?)
                .build()?;

            // Window menu
            let window_menu = SubmenuBuilder::new(app, "Window")
                .item(&PredefinedMenuItem::minimize(app, None)?)
                .item(&separator)
                .item(&PredefinedMenuItem::fullscreen(app, None)?)
                .build()?;

            // Help menu
            let help_menu = SubmenuBuilder::new(app, "Help")
                .build()?;

            let menu = MenuBuilder::new(app)
                .item(&app_menu)
                .item(&file_menu)
                .item(&edit_menu)
                .item(&window_menu)
                .item(&help_menu)
                .build()?;

            app.set_menu(menu)?;

            // Handle menu events
            app.on_menu_event(|_app_handle, event| {
                match event.id().0.as_str() {
                    "open_config_path" => {
                        tauri::async_runtime::spawn(async move {
                            if let Err(e) = commands::open_config_path().await {
                                eprintln!("Failed to open config path: {}", e);
                            }
                        });
                    }
                    _ => {}
                }
            });

            // Initialize app config on startup
            println!("Setting up app...");
            tauri::async_runtime::spawn(async move {
                println!("Initializing app config...");
                match commands::initialize_app_config().await {
                    Ok(()) => println!("App config initialized successfully"),
                    Err(e) => eprintln!("Failed to initialize app config: {}", e),
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            read_config_file,
            write_config_file,
            list_config_files,
            check_app_config_exists,
            create_app_config_dir,
            backup_claude_configs,
            get_stores,
            get_store,
            create_store,
            update_store,
            delete_store,
            set_using_store,
            get_current_store,
            open_config_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
