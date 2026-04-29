# Building On Screen from Source

## Prerequisites

### 1. Rust toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. System dependencies (Linux)

**Ubuntu / Debian / Linux Mint:**

```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  xdotool \
  xprop
```

**Arch / Manjaro:**

```bash
sudo pacman -S --needed \
  webkit2gtk \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  gtk3 \
  xdotool \
  xorg-xprop
```

**Fedora:**

```bash
sudo dnf install -y \
  webkit2gtk4.0-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel \
  xdotool \
  xorg-x11-utils
```

### 3. Node.js (for Tauri CLI)

```bash
# Using nvm (recommended):
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install --lts
nvm use --lts
```

Or grab a package from https://nodejs.org

### 4. Tauri CLI

```bash
npm install
```

---

## Build

```bash
# Development (live reload, opens a window):
npm run dev

# Production build (creates .deb and AppImage in src-tauri/target/release/bundle/):
npm run build
```

Build output will be in:
- `src-tauri/target/release/bundle/deb/` — Debian package
- `src-tauri/target/release/bundle/appimage/` — AppImage

---

## Install built packages

```bash
# AppImage (no install needed, just run):
chmod +x OnScreen_1.0.0_amd64.AppImage
./OnScreen_1.0.0_amd64.AppImage

# .deb:
sudo dpkg -i on-screen_1.0.0_amd64.deb
```

---

## Desktop layer notes

On Screen uses `xdotool` and `xprop` to set `_NET_WM_WINDOW_TYPE_DESKTOP`
on the window after launch. This tells your WM to treat it as a desktop-layer
window — behind all apps, behind icons.

**Tested on:** LXDE, Openbox, XFCE, i3  
**Not tested on:** KDE Plasma (may work), GNOME (likely needs extension)  
**Wayland:** not yet supported — Wayland doesn't expose an equivalent API at this layer without compositor-specific protocols

---

## Autostart on login

Copy the `.desktop` file to your autostart directory:

```bash
mkdir -p ~/.config/autostart
cp on-screen.desktop ~/.config/autostart/
```

Or for LXDE specifically:

```bash
cp on-screen.desktop ~/.config/lxsession/LXDE/autostart
```

---

## Troubleshooting

**App doesn't sit behind windows:**  
Make sure `xdotool` and `xprop` are installed. Run `which xdotool` to confirm.

**Blank window / WebKit error:**  
Install `libwebkit2gtk-4.0-dev` and try again.

**Can't interact with cards:**  
You're in view mode. Press `E` to toggle edit mode. If the toolbar is gone, click anywhere on the canvas and press `E`.

**Board data location:**  
`~/.config/on-screen/` — back this up if you want to preserve your board.
