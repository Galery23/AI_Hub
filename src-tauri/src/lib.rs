// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{
    CustomMenuItem, Menu, MenuItem, Submenu,
    Manager,
    Runtime,
    WindowBuilder,
    WindowUrl,
    window::Window,
};
use std::process::Command;

pub fn create_window<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    // ... existing code ...
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 创建主窗口的菜单
    let file_menu = Submenu::new(
        "文件",
        Menu::new()
            .add_item(CustomMenuItem::new("preferences", "偏好设置"))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("quit", "退出")),
    );

    let edit_menu = Submenu::new(
        "编辑",
        Menu::new()
            .add_item(CustomMenuItem::new("copy", "复制"))
            .add_item(CustomMenuItem::new("paste", "粘贴"))
            .add_item(CustomMenuItem::new("cut", "剪切"))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("selectAll", "全选")),
    );

    let view_menu = Submenu::new(
        "视图",
        Menu::new()
            .add_item(CustomMenuItem::new("fullscreen", "全屏"))
            .add_item(CustomMenuItem::new("reload", "重新加载"))
            .add_native_item(MenuItem::Separator)
            .add_item(CustomMenuItem::new("devtools", "开发者工具")),
    );

    let help_menu = Submenu::new(
        "帮助",
        Menu::new()
            .add_item(CustomMenuItem::new("about", "关于"))
            .add_item(CustomMenuItem::new("documentation", "文档")),
    );

    let main_menu = Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_submenu(view_menu)
        .add_submenu(help_menu);

    tauri::Builder::default()
        .menu(main_menu)
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
                use windows_sys::Win32::UI::WindowsAndMessaging::{GetMenu, HMENU};
                use windows_sys::Win32::Foundation::HWND;
                let main_window = app.get_window("main").unwrap();
                let hwnd = main_window.hwnd().unwrap().0 as HWND;
                unsafe {
                    let menu = GetMenu(hwnd);
                    MAIN_MENU.store(menu as isize, std::sync::atomic::Ordering::SeqCst);
                }
            }
            Ok(())
        })
        .on_window_event(|event| {
            if event.window().label() == "preferences" {
                #[cfg(target_os = "windows")]
                {
                    use windows_sys::Win32::UI::WindowsAndMessaging::SetMenu;
                    use windows_sys::Win32::Foundation::HWND;
                    let hwnd = event.window().hwnd().unwrap().0 as HWND;
                    unsafe {
                        SetMenu(hwnd, 0);
                    }
                }
            }
        })
        .on_menu_event(|event| {
            let win = event.window();
            match event.menu_item_id() {
                "preferences" => {
                    // 创建偏好设置窗口
                    let app = win.app_handle();
                    let preferences_window = app.get_window("preferences");
                    
                    match preferences_window {
                        Some(window) => {
                            // 如果窗口已存在，显示并聚焦
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                        None => {
                            // 如果窗口不存在，创建新窗口
                            WindowBuilder::new(
                                &app,
                                "preferences",
                                WindowUrl::App("index.html#/preferences".into())
                            )
                            .title("偏好设置")
                            .inner_size(800.0, 600.0)
                            .center()
                            .build()
                            .unwrap();
                        }
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                "copy" => {
                    win.eval("document.execCommand('copy')").unwrap();
                }
                "paste" => {
                    win.eval("document.execCommand('paste')").unwrap();
                }
                "cut" => {
                    win.eval("document.execCommand('cut')").unwrap();
                }
                "selectAll" => {
                    win.eval("document.execCommand('selectAll')").unwrap();
                }
                "fullscreen" => {
                    if win.is_fullscreen().unwrap() {
                        win.set_fullscreen(false).unwrap();
                    } else {
                        win.set_fullscreen(true).unwrap();
                    }
                }
                "reload" => {
                    // 重新加载当前窗口
                    win.eval("window.location.reload()").unwrap();
                }
                "devtools" => {
                    // 开发者工具在生产环境中禁用
                    #[cfg(debug_assertions)]
                    println!("开发者工具在生产环境中不可用");
                }
                "about" => {
                    // 创建关于窗口
                    let app = win.app_handle();
                    WindowBuilder::new(
                        &app,
                        "about",
                        WindowUrl::App("index.html#/about".into())
                    )
                    .title("关于")
                    .inner_size(400.0, 300.0)
                    .resizable(false)
                    .build()
                    .unwrap();
                }
                "documentation" => {
                    // 打开文档链接
                    let _ = Command::new("cmd")
                        .args(["/C", "start", "https://github.com/yourusername/yourrepo/wiki"])
                        .spawn();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 用于存储主窗口菜单句柄的静态变量
static MAIN_MENU: std::sync::atomic::AtomicIsize = std::sync::atomic::AtomicIsize::new(0);
