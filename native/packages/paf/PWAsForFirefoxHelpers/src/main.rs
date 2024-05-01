#![windows_subsystem = "windows"]

use tauri_winrt_notification::Toast;
use trayicon::{MenuBuilder, TrayIconBuilder};
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};

#[derive(Clone, Eq, PartialEq)]
#[allow(dead_code)]
enum Events {
    Exit,
    Use,
}

#[allow(deprecated)]
fn main() {
    let events = EventLoop::<Events>::with_user_event().build().unwrap();
    let proxy = events.create_proxy();
    let icon = include_bytes!("icon.ico");

    let sender = move |event: &Events| {
        let _ = proxy.send_event(event.clone());
    };

    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("PWAsForFirefox Portable is running")
        .text1("It is now possible to use it from the extension")
        .text2("You can stop the program from the tray menu")
        .show()
        .unwrap_or_default();

    let mut tray = TrayIconBuilder::new()
        .sender_callback(sender)
        .icon_from_buffer(icon)
        .tooltip("PWAsForFirefox")
        .menu(MenuBuilder::new().item("E&xit", Events::Exit))
        .build()
        .unwrap();

    events
        .run(move |event, target| {
            target.set_control_flow(ControlFlow::Wait);
            let _ = tray;

            if let Event::UserEvent(event) = event {
                match event {
                    Events::Exit => target.exit(),
                    Events::Use => tray.set_menu_item_checkable(Events::Use, true).unwrap(),
                }
            }
        })
        .unwrap();
}
