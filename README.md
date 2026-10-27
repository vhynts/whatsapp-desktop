<div align="center">

<img src="app-icon.svg" width="100" height="100" alt="WhatsApp Desktop Linux">

# WhatsApp Desktop for Ubuntu (Linux)

Klien desktop WhatsApp native yang sangat ringan (~15 MB binary, ~150 MB RAM) untuk Ubuntu / Linux berbasis **Rust** dan **Tauri v2**. Dibuat sebagai alternatif yang efisien untuk wrapper berbasis Electron yang boros memori.

</div>

---

## ✨ Fitur Utama

- **Sangat Ringan & Efisien:** Hanya mengonsumsi RAM sekitar ~150–200 MB (dibandingkan wrapper Electron yang memakan 500 MB – 1 GB+).
- **Background & Tray Icon (AppIndicator):**
  - Icon hijau WhatsApp aktif di system tray (top panel GNOME / taskbar).
  - Klik kiri: Menampilkan / menyembunyikan jendela (toggle).
  - Klik kanan: Menu konteks `Buka WhatsApp` dan `Keluar`.
- **Hide on Close (X):** Menekan tombol silang (Close) tidak mematikan aplikasi, melainkan menyembunyikannya ke background.
- **Native Desktop Notifications:**
  - Terintegrasi langsung dengan GNOME / Freedesktop Notification Server via D-Bus (`notify-rust`).
  - Menampilkan nama kontak dan isi pesan chat masuk.
  - Klik banner notifikasi langsung memunculkan dan memfokuskan jendela WhatsApp.
- **Single Instance Lock:** Mencegah pembukaan jendela ganda secara tidak sengaja. Membuka aplikasi dari launcher saat sudah berjalan akan langsung memfokuskan jendela yang sudah ada.
- **Offline & No-Internet Handling:** Tampilan offline elegan bergaya tema gelap WhatsApp dengan tombol *Coba Lagi* serta pendeteksi otomatis (*auto-reconnect*) ketika jaringan internet kembali pulih.
- **Modern User-Agent:** Disesuaikan dengan Google Chrome Linux terbaru sehingga WhatsApp Web selalu berfungsi tanpa peringatan browser tidak didukung.

---

## ⚠️ Batasan (Limitations)

- **Voice & Video Call:** Tidak tersedia. WhatsApp/Meta secara resmi mengunci fitur panggilan WebRTC hanya untuk aplikasi resmi di Windows dan macOS. Batasan ini sama seperti saat menggunakan WhatsApp Web di browser apa pun di Linux.
- **Audio Voice Note & Media:** Berjalan 100% normal (didukung codec GStreamer).

---

## 📦 Prasyarat Sistem (Ubuntu 24.04 / 26.04)

Paket dependensi sistem yang dibutuhkan untuk build:

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly
```

Dan Rust toolchain:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## 🚀 Cara Build & Menjalankan

### Mode Development
```bash
cd src-tauri
cargo run
```

### Build Binary Release
```bash
cd src-tauri
cargo build --release
```
Binary hasil kompilasi akan berada di:
`src-tauri/target/release/whatsapp-desktop` (ukuran ~15 MB).

---

## 💻 Instalasi ke Sistem Ubuntu

Agar aplikasi terdaftar di menu aplikasi ("Show Apps") dan PATH sistem:

1. **Salin binary ke `~/.local/bin`:**
   ```bash
   cp src-tauri/target/release/whatsapp-desktop ~/.local/bin/
   chmod +x ~/.local/bin/whatsapp-desktop
   ```

2. **Pasang Icon Aplikasi:**
   ```bash
   mkdir -p ~/.local/share/icons/hicolor/{32x32,64x64,128x128,256x256,512x512,scalable}/apps
   cp src-tauri/icons/32x32.png ~/.local/share/icons/hicolor/32x32/apps/whatsapp-desktop.png
   cp src-tauri/icons/64x64.png ~/.local/share/icons/hicolor/64x64/apps/whatsapp-desktop.png
   cp src-tauri/icons/128x128.png ~/.local/share/icons/hicolor/128x128/apps/whatsapp-desktop.png
   cp src-tauri/icons/128x128@2x.png ~/.local/share/icons/hicolor/256x256/apps/whatsapp-desktop.png
   cp src-tauri/icons/icon.png ~/.local/share/icons/hicolor/512x512/apps/whatsapp-desktop.png
   cp app-icon.svg ~/.local/share/icons/hicolor/scalable/apps/whatsapp-desktop.svg
   ```

3. **Buat Desktop Entry (`~/.local/share/applications/whatsapp-desktop.desktop`):**
   ```ini
   [Desktop Entry]
   Type=Application
   Name=WhatsApp
   Comment=WhatsApp Desktop Client
   GenericName=WhatsApp
   Exec=/home/anang/.local/bin/whatsapp-desktop %u
   Icon=/home/anang/.local/share/icons/hicolor/512x512/apps/whatsapp-desktop.png
   Terminal=false
   Categories=Network;InstantMessaging;Chat;
   Keywords=WhatsApp;chat;im;messaging;
   StartupNotify=true
   StartupWMClass=whatsapp-desktop
   MimeType=x-scheme-handler/whatsapp;
   ```

4. **Perbarui Database Desktop:**
   ```bash
   update-desktop-database ~/.local/share/applications
   ```

---

## 🛠️ Arsitektur & Cara Kerja

- **Tauri v2:** Kerangka kerja native yang mengikat WebKitGTK dengan backend Rust tanpa overhead runtime Node.js.
- **Ayatana AppIndicator:** Menggunakan standar D-Bus StatusNotifierItem Linux untuk icon tray dan menu konteks.
- **WebKitGTK Event Hooks:**
  - `connect_permission_request`: Mengizinkan notifikasi web dan media secara otomatis.
  - `connect_show_notification`: Mencegat event notifikasi pesan WhatsApp Web dan merutekannya ke `notify-rust` untuk ditampilkan sebagai notifikasi native Ubuntu.
- **Single Instance Listener:** Menggunakan socket/D-Bus internal untuk memfokuskan jendela yang sudah berjalan jika binary dipanggil kembali.

---

## 📄 Lisensi

MIT License. WhatsApp adalah merek dagang terdaftar milik Meta Platforms, Inc. Proyek ini adalah wrapper tidak resmi pihak ketiga.
