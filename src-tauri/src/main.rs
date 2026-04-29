// On Screen — Tauri shell
// Sets up a frameless, borderless window at the X11 desktop layer
// (_NET_WM_WINDOW_TYPE_DESKTOP) so the OS treats it as wallpaper
// while still receiving pointer events when no other window is on top.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowBuilder, WindowUrl};

#[cfg(target_os = "linux")]
use std::process::Command;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();

            // Remove decorations and make fullscreen
            win.set_decorations(false).ok();
            win.set_fullscreen(true).ok();

            // Always below other windows at the OS level
            win.set_always_on_bottom(true).ok();

            // X11: register as desktop-type window so WMs treat it correctly
            #[cfg(target_os = "linux")]
            set_desktop_window_type(&win);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running On Screen");
}

/// Use xdotool or xprop to set _NET_WM_WINDOW_TYPE_DESKTOP on the window.
/// This tells X11 window managers (LXDE, Openbox, XFCE, i3, etc.) that
/// this window is a desktop layer — it sits behind icons and all apps.
#[cfg(target_os = "linux")]
fn set_desktop_window_type(win: &tauri::Window) {
    // Give the window a moment to be mapped before querying its ID
    std::thread::sleep(std::time::Duration::from_millis(400));

    // Get window ID via xdotool (most reliable cross-WM method)
    if let Ok(output) = Command::new("xdotool")
        .args(["search", "--name", "On Screen"])
        .output()
    {
        let wid = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !wid.is_empty() {
            // Set window type to DESKTOP
            Command::new("xprop")
                .args([
                    "-id", &wid,
                    "-f", "_NET_WM_WINDOW_TYPE", "32a",
                    "-set", "_NET_WM_WINDOW_TYPE", "_NET_WM_WINDOW_TYPE_DESKTOP",
                ])
                .output()
                .ok();

            // Remove from taskbar and pager
            Command::new("xprop")
                .args([
                    "-id", &wid,
                    "-f", "_NET_WM_STATE", "32a",
                    "-set", "_NET_WM_STATE",
                    "_NET_WM_STATE_SKIP_TASKBAR,_NET_WM_STATE_SKIP_PAGER",
                ])
                .output()
                .ok();
        }
    }
}
