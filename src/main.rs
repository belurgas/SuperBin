// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{ptr::null_mut, thread, time::Duration};

use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem}, 
    MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent
};
use winapi::{shared::winerror::S_OK, um::{shellapi::{SHEmptyRecycleBinW, SHQueryRecycleBinW, ShellExecuteW}, winuser::{HWND_DESKTOP, SW_SHOWNORMAL}}};
use winit::{
    application::ApplicationHandler,
    event_loop::EventLoop,
};
use wio::wide::ToWide;

#[derive(Debug)]
enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
    UpdateTooltip(String), // Добавлено новое событие для обновления тултипа
}

struct Application {
    tray_icon: Option<TrayIcon>,
}

impl Application {
    fn new() -> Application {
        Application { tray_icon: None }
    }

    fn new_tray_icon() -> TrayIcon {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/trash-bin.ico");
        let icon = load_icon(std::path::Path::new(path));

        TrayIconBuilder::new()
            .with_menu(Box::new(Self::new_tray_menu()))
            .with_menu_on_left_click(false)
            .with_tooltip("Корзиныч")
            .with_icon(icon)
            .with_title("x")
            .build()
            .unwrap()
    }

    fn new_tray_menu() -> Menu {
        let menu = Menu::new();

        let item1 = MenuItem::new("Открыть корзину", true, None);
        menu.append(&item1).unwrap();

        let item_close = MenuItem::new("Выйти", true, None);
        menu.append(&item_close).unwrap();

        let clear = MenuItem::new("Отчистить корзину", true, None);
        menu.append(&clear).unwrap();

        menu
    }
}

impl ApplicationHandler<UserEvent> for Application {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        // We create the icon once the event loop is actually running
        // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
        if winit::event::StartCause::Init == cause {
            #[cfg(not(target_os = "linux"))]
            {
                self.tray_icon = Some(Self::new_tray_icon());
            }

            // We have to request a redraw here to have the icon actually show up.
            // Winit only exposes a redraw method on the Window so we use core-foundation directly.
            #[cfg(target_os = "macos")]
            unsafe {
                use objc2_core_foundation::{CFRunLoopGetMain, CFRunLoopWakeUp};

                let rl = CFRunLoopGetMain().unwrap();
                CFRunLoopWakeUp(&rl);
            }
        }
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::TrayIconEvent(tray_icon_event) => {
                match tray_icon_event {
                    TrayIconEvent::Click { button, .. } if button == MouseButton::Left => {
                        open_recycle_bin();
                    }
                    _ => {}
                }
            },
            UserEvent::MenuEvent(menu_event) => {
                match menu_event.id.0.as_str() {
                    "1001" => open_recycle_bin(),
                    "1002" => std::process::exit(0),
                    "1003" => {
                        if empty_recycle_bin() {
                            println!("Корзина очищена");

                            // Можно обновить тултип
                            if let Some(tray) = &self.tray_icon {
                                tray.set_tooltip(Some("Корзина пуста")).unwrap();
                            }
                        } else {
                            println!("Ошибка при очистке корзины");
                        }
                    }
                    _ => {}
                }
            },
            UserEvent::UpdateTooltip(tip) => { // Обработка нового события
                if let Some(tray) = &self.tray_icon {
                    tray.set_tooltip(Some(&tip)).unwrap();
                }
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    // Регистрируем обработчики событий
    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::TrayIconEvent(event)).unwrap();
    }));
    
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        proxy.send_event(UserEvent::MenuEvent(event)).unwrap();
    }));

    let mut app = Application::new();

    // Запускаем поток для обновления тултипа
    let proxy = event_loop.create_proxy();
    thread::spawn(move || {
        loop {
            let size = get_recycle_bin_size();
            let tip = format!("Корзина: {:.2} МБ", size as f64 / 1_000_000.0);
            
            // Отправляем событие в основной поток
            proxy.send_event(UserEvent::UpdateTooltip(tip)).unwrap();
            
            thread::sleep(Duration::from_secs(10));
        }
    });

    event_loop.run_app(&mut app).unwrap();
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

// Функция открытия корзины Windows
fn open_recycle_bin() {
    unsafe {
        let verb = "open".to_wide_null();
        let file = "shell:RecycleBinFolder".to_wide_null();
        ShellExecuteW(
            null_mut(),
            verb.as_ptr(),
            file.as_ptr(),
            null_mut(),
            null_mut(),
            SW_SHOWNORMAL,
        );
    }
}

fn get_recycle_bin_size() -> u64 {
    unsafe {
        let mut info = winapi::um::shellapi::SHQUERYRBINFO {
            cbSize: std::mem::size_of::<winapi::um::shellapi::SHQUERYRBINFO>() as u32,
            i64Size: 0,
            i64NumItems: 0,
        };

        let result = SHQueryRecycleBinW(null_mut(), &mut info);
            
        if result == S_OK {
            return info.i64Size.try_into().unwrap();
        } else {
            println!("Ошибка при запросе размера корзины: {}", result);
            return 0;
        }
    }
}

fn empty_recycle_bin() -> bool {
    unsafe {
        let result = SHEmptyRecycleBinW(
            HWND_DESKTOP,
            null_mut(),
            0
        );
        result == S_OK
    }
}