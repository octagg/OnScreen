# On Screen

> *"On screen." — what the captain says when something needs to be seen.*

An interactive desktop pinboard that lives at the OS desktop layer — behind your windows, always available, never in the way. Pin notes, reference images, hotkey maps, and shortcuts directly onto your desktop. Toggle edit mode and rearrange everything. Switch to view mode and it disappears into the background.

![On Screen — LCARS theme](docs/screenshot-lcars.png)

---

## What it does

- **Runs at the desktop layer** — the OS treats it as wallpaper. It sits behind all your application windows.
- **Text cards** — inline editing, titled, color-coded, resizable, moveable
- **Image cards** — paste from clipboard, drop a file, or enter a URL. Resizable and moveable like text cards.
- **Edit / View mode** — `E` to toggle. In view mode the toolbar disappears and the board is read-only.
- **Infinite canvas** — pan with `Space + drag` or middle-click drag
- **Full theme system** — four bundled themes, simple color picker, full CSS var editor, save custom themes, random palette generator
- **Export / Import** — your board is a plain JSON file. Back it up, version control it, share layouts.
- **Persistent** — state saves automatically. Your board survives reboots.
- **Zero dependencies** — single HTML file core, no frameworks, no CDN calls

---

## Themes

| LCARS | VANILLA | PAPER | TERMINAL |
|-------|---------|-------|----------|
| Amber on black | Minimal dark | Light / warm | Greyscale mono |

Not a Trek fan? **Settings → THEMES → VANILLA** takes 3 seconds. Or build your own with the color picker or raw CSS var editor. The MIX button generates a random harmonious palette.

---

## Keyboard shortcuts

| Key | Action |
|-----|--------|
| `E` | Toggle edit / view mode |
| `T` | New text card |
| `I` | New image card (prompts for URL) |
| `,` | Open / close settings |
| `Space + drag` | Pan the canvas |
| `Del` | Delete selected card |
| `Escape` | Close settings panel |

---

## Installation

### AppImage (recommended)

```bash
chmod +x OnScreen-x86_64.AppImage
./OnScreen-x86_64.AppImage
```

### .deb (Debian / Ubuntu / Mint)

```bash
sudo dpkg -i onscreen_1.0.0_amd64.deb
```

### From source

Requires [Rust](https://rustup.rs) and the [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites).

```bash
git clone https://github.com/octagg/OnScreen.git
cd on-screen
cargo tauri build
```

---

## Desktop layer setup (Linux / X11)

On Screen uses `_NET_WM_WINDOW_TYPE_DESKTOP` to register itself with your window manager as a desktop-layer window. This is handled automatically by the Tauri shell on launch.

**Tested on:** LXDE, Openbox, XFCE, i3  
**Wayland:** planned for a future release  
**Windows:** community port welcome — see [CONTRIBUTING.md](CONTRIBUTING.md)

---

## Settings

### Appearance
- Greyscale filter (full app)
- Monochrome cards (removes color variants)
- Grid density — normal / fine / coarse / off
- Drop shadows
- Console sound (TNG chirp — yes, really)

### Themes
- 4 bundled themes
- Simple color picker — 5 colors, borders and grid auto-derived
- Advanced CSS var editor — full control over all 11 theme variables
- Save, name, and recall custom themes
- Random palette mixer

### Effects (all off by default)
- CSS transitions
- Card appear animation
- Drag lift shadow
- Hover glow
- Ambient background pulse

### Cards
- Body font size
- Corner radius
- Default new card color
- Drag strip height

---

## Board export / import

Your board data is portable JSON:

```json
{
  "version": "1.0.0",
  "app": "On Screen",
  "exported": "2026-04-29T00:00:00.000Z",
  "board": { "cards": [...], "pan": { "x": 0, "y": 0 } },
  "settings": { ... },
  "theme": { "id": "lcars", "vars": { ... } },
  "customThemes": { ... }
}
```

Use `EXPORT` in the toolbar or Settings → Appearance to download. `IMPORT` to restore. Works across machines.

---

## Philosophy

On Screen is free and open source. It solves a real problem that no existing tool solved cleanly — an interactive, persistent reference layer at the desktop layer on Linux. If it helps you, a small donation keeps it maintained. If you can't or won't donate, use it anyway. That's fine.

This is a giveback for all the wonderful things others have done for the community.

---

## Contributing

Issues and PRs welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).  
If you're working on a Windows port, open an issue first so we can coordinate on the WorkerW approach.

---

## License

MIT — do what you want, attribution appreciated.

---

## Donate

If On Screen saves you time or brain cycles, consider buying the maintainer a coffee.

**[Ko-fi](https://ko-fi.com/octagg)** · **[GitHub Sponsors](https://github.com/sponsors/octagg)** *(pending approval)*

---

*Named for what the captain says when the lieutenant says something is going on.*
