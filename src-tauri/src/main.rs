// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_os = "linux")]
fn setup_filtered_dbus() -> Option<(std::process::Child, String)> {
    if std::env::var("WHATSAPP_FILTERED_BUS").is_ok() {
        return None;
    }

    let real_bus = std::env::var("DBUS_SESSION_BUS_ADDRESS").ok()?;
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
    let socket_path = format!("{}/whatsapp-desktop-bus.sock", runtime_dir);

    let _ = std::fs::remove_file(&socket_path);

    let child = std::process::Command::new("xdg-dbus-proxy")
        .arg(&real_bus)
        .arg(&socket_path)
        .arg("--filter")
        .arg("--sloppy-names")
        .arg("--talk=org.freedesktop.*")
        .arg("--talk=org.gnome.*")
        .arg("--talk=org.kde.*")
        .arg("--talk=org.gtk.*")
        .arg("--talk=org.a11y.*")
        .arg("--talk=com.canonical.*")
        .arg("--see=org.freedesktop.*")
        .arg("--see=org.gnome.*")
        .arg("--see=org.kde.*")
        .arg("--see=org.gtk.*")
        .arg("--see=org.a11y.*")
        .arg("--see=com.canonical.*")
        .arg("--own=org.ayatana.NotificationItem.*")
        .arg("--own=com.whatsapp.desktop.*")
        .spawn()
        .ok()?;

    for _ in 0..20 {
        if std::path::Path::new(&socket_path).exists() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }

    if std::path::Path::new(&socket_path).exists() {
        unsafe {
            std::env::set_var(
                "DBUS_SESSION_BUS_ADDRESS",
                format!("unix:path={}", socket_path),
            );
            std::env::set_var("WHATSAPP_FILTERED_BUS", "1");
        }
        Some((child, socket_path))
    } else {
        None
    }
}

fn main() {
    #[cfg(target_os = "linux")]
    let proxy_guard = setup_filtered_dbus();

    app_lib::run();

    #[cfg(target_os = "linux")]
    if let Some((mut child, sock)) = proxy_guard {
        let _ = child.kill();
        let _ = child.wait();
        let _ = std::fs::remove_file(sock);
    }
}
