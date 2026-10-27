#!/usr/bin/env bash
set -euo pipefail

echo "==> WhatsApp Desktop Installer for Ubuntu <=="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_SRC="${SCRIPT_DIR}/src-tauri/target/release/whatsapp-desktop"
ICON_SRC="${SCRIPT_DIR}/src-tauri/icons"
APP_DIR="${HOME}/.local/bin"
ICON_DIR="${HOME}/.local/share/icons/hicolor"
DESKTOP_DIR="${HOME}/.local/share/applications"

# 1. Build if binary doesn't exist
if [[ ! -f "${BIN_SRC}" ]]; then
    echo "==> Building binary with Cargo..."
    cd "${SCRIPT_DIR}/src-tauri"
    cargo build --release
    cd "${SCRIPT_DIR}"
fi

# 2. Install binary
echo "==> Installing binary to ${APP_DIR}..."
mkdir -p "${APP_DIR}"
cp "${BIN_SRC}" "${APP_DIR}/whatsapp-desktop"
chmod +x "${APP_DIR}/whatsapp-desktop"

# 3. Install icons
echo "==> Installing icons to ${ICON_DIR}..."
mkdir -p "${ICON_DIR}"/{32x32,64x64,128x128,256x256,512x512,scalable}/apps
cp "${ICON_SRC}/32x32.png" "${ICON_DIR}/32x32/apps/whatsapp-desktop.png"
cp "${ICON_SRC}/64x64.png" "${ICON_DIR}/64x64/apps/whatsapp-desktop.png"
cp "${ICON_SRC}/128x128.png" "${ICON_DIR}/128x128/apps/whatsapp-desktop.png"
cp "${ICON_SRC}/128x128@2x.png" "${ICON_DIR}/256x256/apps/whatsapp-desktop.png"
cp "${ICON_SRC}/icon.png" "${ICON_DIR}/512x512/apps/whatsapp-desktop.png"
cp "${SCRIPT_DIR}/app-icon.svg" "${ICON_DIR}/scalable/apps/whatsapp-desktop.svg"

# Symlinks for whatsapp -> whatsapp-desktop
for res in 32x32 64x64 128x128 256x256 512x512; do
    ln -sf "whatsapp-desktop.png" "${ICON_DIR}/${res}/apps/whatsapp.png"
done
ln -sf "whatsapp-desktop.svg" "${ICON_DIR}/scalable/apps/whatsapp.svg"

# 4. Install Desktop Entry
echo "==> Installing desktop entry..."
mkdir -p "${DESKTOP_DIR}"
cat <<EOF > "${DESKTOP_DIR}/whatsapp-desktop.desktop"
[Desktop Entry]
Type=Application
Name=WhatsApp
Comment=WhatsApp Desktop Client
GenericName=WhatsApp
Exec=${APP_DIR}/whatsapp-desktop %u
Icon=${ICON_DIR}/512x512/apps/whatsapp-desktop.png
Terminal=false
Categories=Network;InstantMessaging;Chat;
Keywords=WhatsApp;chat;im;messaging;
StartupNotify=true
StartupWMClass=whatsapp-desktop
MimeType=x-scheme-handler/whatsapp;
EOF

chmod +x "${DESKTOP_DIR}/whatsapp-desktop.desktop"
update-desktop-database "${DESKTOP_DIR}" || true

echo "==> Installation complete! You can run 'whatsapp-desktop' or find WhatsApp in Show Apps."
