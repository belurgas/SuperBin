// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{fs::File, io::Read, path::Path, ptr::null_mut, thread, time::Duration};

use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem}, Icon, MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent
};
use winapi::{shared::winerror::S_OK, um::{shellapi::{SHEmptyRecycleBinW, SHQueryRecycleBinW, ShellExecuteW}, winnt::{KEY_READ, KEY_SET_VALUE}, winuser::{HWND_DESKTOP, SW_SHOWNORMAL}}};
use winit::{
    application::ApplicationHandler,
    event_loop::EventLoop,
};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};
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
        // let icon = load_icon(std::path::Path::new(path));

        // Встраиваем уже подготовленные RGBA-данные
        let rgba_data = include_bytes!("ds.rgba");

        // Проверяем, что длина соответствует 32x32 RGBA
        assert_eq!(rgba_data.len(), 48 * 48 * 4);

        // Создаём иконку
        let icon = Icon::from_rgba(rgba_data.to_vec(), 48, 48).expect("Не удалось создать иконку");

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

        let clear = MenuItem::new("Отчистить корзину", true, None);
        menu.append(&clear).unwrap();

        let item_close = MenuItem::new("Выйти", true, None);
        menu.append(&item_close).unwrap();

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
                    "1003" => std::process::exit(0),
                    "1002" => {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Проверяем, есть ли уже в автозагрузке
    if !check_startup()? {
        println!("Приложение не в автозагрузке. Добавляем...");
        add_to_startup()?;
    } else {
        println!("Приложение уже в автозагрузке.");
    }

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

    Ok(())
}

// Простая реализация загрузки иконки .ico (просто берёт первое изображение)
// fn load_icon<P: AsRef<Path>>(path: P) -> Icon {
//     let mut file = File::open(path).expect("Не удалось открыть файл");
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer).expect("Не удалось прочитать файл");

//     // Пропускаем заголовок ICO файла (6 байт), переходим к таблице изображений
//     let entry_offset = u32::from_le_bytes([
//         buffer[4], buffer[5], 0, 0,
//     ]) as usize;

//     // Прыгаем на начало первого изображения
//     let image_data = &buffer[entry_offset..];

//     // Предполагаем, что изображение 32x32 и имеет RGBA формат (не всегда верно!)
//     const width: u32 = 32;
//     const height: u32 = 32;

//     // Берём первые 32x32x4 байт как RGBA
//     let rgba: Vec<u8> = image_data[..(width * height * 4) as usize]
//         .chunks_exact(4)
//         .flat_map(|p| [p[2], p[1], p[0], p[3]]) // Bgra → Rgba
//         .collect();

//     Icon::from_rgba(rgba, width, height).expect("Не удалось создать иконку")
// }

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

fn add_to_startup() -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run", 
        KEY_SET_VALUE,
    )?;

    let exe_path = std::env::current_exe()?;
    let path_str = exe_path.to_str().ok_or("Невозможно преобразовать путь")?;

    run_key.set_value("minibin", &format!("\"{}\"", path_str))?;
    println!("Приложение добавлено в автозагрузку.");

    Ok(())
}

fn check_startup() -> Result<bool, Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run_key = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_READ,
    )?;

    let result: Result<String, _> = run_key.get_value("minibin");
    if let Ok(path) = result {
        let current_exe = std::env::current_exe()?;
        let current_path = current_exe.to_str().ok_or("Ошибка пути")?;
        return Ok(path.contains(current_path));
    }

    Ok(false)
}