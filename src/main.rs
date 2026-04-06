#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use clap::Parser;
use dotenvy::dotenv;

use richaudbot::{Args, start_bot, try_attach_console};

mod tray;

use tokio::runtime::Runtime;
use tokio_util::sync::CancellationToken;
use tray_icon::{menu::{MenuEvent, MenuId}};

use crate::tray::{setup_tray};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    try_attach_console();

    let args = Args::parse();
    dotenv().expect("Unable to find .env");

    let _tray_icon = setup_tray();

    let rt = Runtime::new()?; 

    let cancel_token = CancellationToken::new();

    let bot_token = cancel_token.clone();

    let ctrlc_token = cancel_token.clone();

    ctrlc::set_handler(move || {
        ctrlc_token.cancel();
    }).expect("Error with Ctrl+C handler setup");

    rt.spawn(async move {
        println!("Starting Richaud in background...");
        start_bot(args, bot_token).await;
    });

    let menu_channel = MenuEvent::receiver();
    
    loop {
        if cancel_token.is_cancelled() {
            break;
        }
        #[cfg(target_os = "windows")]
        unsafe {
            use windows_sys::Win32::UI::WindowsAndMessaging::{
                DispatchMessageW, PeekMessageW, TranslateMessage, PM_REMOVE,
            };
            let mut msg = std::mem::zeroed();
            while PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == MenuId::from(1000) {
                cancel_token.cancel();
                break;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    rt.shutdown_timeout(std::time::Duration::from_secs(3));

    Ok(())
}

