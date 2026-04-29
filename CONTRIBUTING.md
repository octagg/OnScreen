# Contributing to On Screen

Thanks for taking the time. On Screen is a small open source project maintained by one person — contributions that are focused and purposeful are genuinely appreciated.

---

## Before you open a PR

- Check the [issues](https://github.com/octagg/OnScreen/issues) to see if it's already being discussed
- For anything beyond a small bug fix, open an issue first so we can agree on direction before you spend time writing code
- Keep PRs focused — one thing per PR

---

## What's welcome

- Bug fixes
- Compatibility improvements (different WMs, distros)
- Performance improvements — On Screen runs full time, idle cost matters
- Documentation improvements
- Translations (if there's ever a UI string file to translate)
- Windows WorkerW port — open an issue first to coordinate

## What to avoid

- Adding dependencies — the zero-dependency single HTML file is a feature, not a limitation
- Framework rewrites
- Feature additions that add idle CPU/GPU cost without a toggle to disable them
- Anything that breaks the lean-by-default philosophy

---

## Development setup

See [INSTALL.md](INSTALL.md) for full prerequisites.

```bash
git clone https://github.com/octagg/OnScreen.git
cd OnScreen/app
npm install
npm run dev
```

The dev build opens a Tauri window with live reload. Most work happens in `app/src/index.html` — the entire app UI lives in that one file.

---

## Code style

- No build step for the frontend — vanilla HTML, CSS, JS only
- No external JS libraries or CDN calls
- CSS variables for all theme values — new visual properties go through the var system
- Effects go in the effects section, gated by a body class, off by default
- Comments where the intent isn't obvious — especially in the Rust shell

---

## Reporting bugs

Open an issue with:
- Your distro and WM
- Steps to reproduce
- What you expected vs what happened
- Any console errors (F12 in the Tauri window during dev)

---

## Donations

If you contribute and use the app, a Ko-fi is always appreciated but never expected.  
[https://ko-fi.com/octagg](https://ko-fi.com/octagg)
