use std::{sync::mpsc, thread};

use sysinfo::{Components, Disks, System};
use tauri::{menu::{Menu, MenuItem}, App, Emitter, Manager};
use tauri_plugin_positioner::{Position, WindowExt};

// 1. Информация о дисках
#[tauri::command]
fn get_disks_info() -> Vec<(String, String, u64, u64)> {
    let disks = Disks::new_with_refreshed_list();
    disks
        .list()
        .iter()
        .map(|disk| {
            let letter = disk.mount_point().to_string_lossy().into_owned();
            let name = disk.name().to_string_lossy().into_owned();
            let total = disk.total_space();
            let free = disk.available_space();
            (letter, name, total, free)
        })
        .collect()
}

#[tauri::command]
fn system_info() -> (String, u64, u64) {
    let platform = tauri_plugin_os::platform();
    let data: (String, u64, u64) = {
        let mut sys = System::new_all();
        sys.refresh_all();
        (
            platform.to_string(),
            sys.total_memory(),
            sys.used_memory(),
        )
    };
    return data;
}

#[tauri::command]
fn get_temperatures() -> Vec<(String, f32)> {
    let components = Components::new_with_refreshed_list();
    let mut temps: Vec<(String, f32)> = vec![];
    for component in &components {
        if let Some(temperature) = component.temperature() {
            temps.push((component.label().to_string(), temperature));
        }
    }
    println!("TEMPS: {:?}", temps);
    temps
}

fn monitor_memory(tx: mpsc::Sender<u64>) {
    let mut system = System::new_all();
    loop {
        system.refresh_all();
        let used_memory = system.used_memory(); // В килобайтах
        tx.send(used_memory).unwrap();
        thread::sleep(std::time::Duration::from_secs(1)); // Проверка каждую секунду
    }
}


fn tray_icon(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::tray::{TrayIcon, TrayIconBuilder};

    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;

    let tray = TrayIconBuilder::new()
    .menu(&menu)
    .show_menu_on_left_click(true)
    .on_menu_event(|app, event| match event.id.as_ref() {
        "quit" => {
        println!("quit menu item was clicked");
        app.exit(0);
        }
        _ => {
        println!("menu item {:?} not handled", event.id);
        }
    })
    .icon(app.default_window_icon().unwrap().clone()).build(app)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (tx, rx) = mpsc::channel();

    // Запуск потока для мониторинга памяти
    thread::spawn(move || {
        monitor_memory(tx);
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_disks_info,
            get_temperatures,
            system_info
        ])
        .setup(|app| {
            
            let tray = tray_icon(app).unwrap();

            let window = app.get_webview_window("main").unwrap();
            let _ = window.as_ref().window().move_window(Position::Center);
            tauri::async_runtime::spawn(async move {
                while let Ok(used_memory) = rx.recv() {
                    window.emit("memory-update", used_memory).unwrap();
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
