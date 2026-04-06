use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem}};

pub fn setup_tray() -> tray_icon::TrayIcon {
    let quit = MenuItem::new("Stop", true, None);

    let tray_menu = Menu::new();
    tray_menu.append_items(&[&quit]).unwrap();

    let icon = load_icon();

    TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("RICHAUD PRIME")
        .with_icon(icon)
        .build()
        .unwrap()
}

fn load_icon() -> tray_icon::Icon {
    let icon_bytes = include_bytes!("../icon.png");
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(icon_bytes).expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

/*pub fn set_console_visibility(visible: bool) {
    unsafe {
        let window = GetConsoleWindow();
        if window != std::ptr::null_mut() {
            let cmd = if visible { SW_SHOW } else { SW_HIDE };
            ShowWindow(window, cmd);
        }
    }
} **/