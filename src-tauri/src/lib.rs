use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, RunEvent, WindowEvent,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Keluar", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Buka WhatsApp", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let icon_bytes = include_bytes!("../icons/tray.png");
            let icon = Image::from_bytes(icon_bytes)?;

            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .tooltip("WhatsApp")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.unminimize();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            #[cfg(target_os = "linux")]
            if let Some(window) = app.get_webview_window("main") {
                use webkit2gtk::{NotificationExt, PermissionRequestExt, WebViewExt};
                let win_for_click = window.clone();
                let _ = window.with_webview(move |webview| {
                    let wv = webview.inner();

                    // Automatically allow notification and media permission requests
                    wv.connect_permission_request(|_wv, req| {
                        req.allow();
                        true
                    });

                    // Intercept web notifications and display them on Ubuntu desktop
                    let win_target = win_for_click.clone();
                    wv.connect_show_notification(move |_wv, notif| {
                        let title = notif
                            .title()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "WhatsApp".to_string());
                        let body = notif.body().map(|s| s.to_string()).unwrap_or_default();

                        let win_handle = win_target.clone();
                        std::thread::spawn(move || {
                            let handle = notify_rust::Notification::new()
                                .appname("WhatsApp")
                                .summary(&title)
                                .body(&body)
                                .icon("whatsapp-desktop")
                                .action("default", "Buka")
                                .show();

                            if let Ok(handle) = handle {
                                handle.wait_for_action(move |action| {
                                    if action == "default" || action == "clicked" {
                                        let _ = win_handle.show();
                                        let _ = win_handle.unminimize();
                                        let _ = win_handle.set_focus();
                                    }
                                });
                            }
                        });

                        true
                    });
                });

                // Periodically prompt for notification permission in webview after loading
                let win_eval = window.clone();
                std::thread::spawn(move || {
                    for _ in 0..5 {
                        std::thread::sleep(std::time::Duration::from_secs(3));
                        let _ = win_eval.eval(
                            "if (window.Notification && Notification.permission !== 'granted') { Notification.requestPermission(); }"
                        );
                    }
                });
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_app_handle, event| {
        if let RunEvent::ExitRequested { code, api, .. } = event {
            if code.is_none() {
                api.prevent_exit();
            }
        }
    });
}
