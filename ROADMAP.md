# On Screen — Roadmap

This is a living document. Things move based on real usage, bug reports, and what turns out to actually matter.

---

## v1.0.0 — shipped

- [x] Interactive canvas — draggable, resizable, titled cards
- [x] Text cards with inline editing
- [x] Image cards — URL, clipboard paste, file drop
- [x] Edit / view mode toggle
- [x] Infinite canvas with pan
- [x] Full theme system — 4 bundled themes
- [x] Simple color picker — accent-derived borders and grid
- [x] Advanced CSS var editor
- [x] Save / load custom themes
- [x] Random palette mixer
- [x] Effects system — all off by default, individually toggled
- [x] Export / import board as JSON
- [x] Persistent state
- [x] Console sound (TNG chirp)
- [x] Tauri shell with X11 desktop layer registration

---

## v1.1 — quality pass

- [ ] Snap to grid option
- [ ] Card z-order controls (bring to front / send to back)
- [ ] Duplicate card
- [ ] Multi-select and group move
- [ ] Right-click context menu
- [ ] Undo / redo (Ctrl+Z)
- [ ] Board title / name

---

## v1.2 — content

- [ ] Markdown rendering in text cards (toggleable)
- [ ] Link cards — URL with favicon and title preview
- [ ] Code cards — monospace, syntax highlight optional
- [ ] Checklist cards

---

## v1.3 — system integration

- [ ] Global hotkey to toggle edit mode from outside the app
- [ ] System tray icon with quick actions
- [ ] Autostart on login (`.desktop` entry, packaged)
- [ ] Multiple boards — switch between named boards
- [ ] Board per workspace / virtual desktop

---

## v2.0 — platform

- [ ] Wayland support
- [ ] Windows port (WorkerW layer — community contribution welcome)
- [ ] macOS investigation

---

## Icebox — not planned but not ruled out

- Sync between machines (Syncthing-compatible board files)
- Plugin / widget system
- Drawing / freehand annotation layer
- Collaborative boards

---

## Philosophy

On Screen stays lean by default. Every new feature gets evaluated against:

1. Does it add idle resource cost? If yes, it needs a toggle that defaults off.
2. Does it add a dependency? If yes, the bar is very high.
3. Does it complicate the single-file architecture? If yes, is it worth it?

The goal is a tool that runs full time on a dedicated monitor and costs essentially nothing when you're not actively using it.

---

*Suggestions welcome — open an issue or start a discussion on GitHub.*
