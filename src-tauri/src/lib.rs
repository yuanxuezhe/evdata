#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri_plugin_autostart::ManagerExt;
use std::env;

#[tauri::command]
// 获取exe所在路径
fn get_exe_path() -> Result<String, String> {
    env::current_exe()
        .map(|path| path.to_string_lossy().to_string())
        .map_err(|err| err.to_string())
}

#[tauri::command]
// 获取配置文件所在路径
fn get_plugin_path() -> Result<String, String> {
    env::current_exe()
        .map(|mut path| {
            path.pop(); // 移除可执行文件名，保留目录路径
            path.push("plugin"); // 添加 plugin 文件夹
            path.to_string_lossy().to_string()
        })
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn my_custom_command() -> String {
    let ans = "I was invoked from JavaScript!";
    println!("{}", ans);
    ans.to_string()
}

#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello, {}!", name).to_string()
}

// 测试模块
#[cfg(test)]
mod tests {
    // 将外部作用域的所有内容引入到测试模块中
    use super::*;

    // 另一个测试函数
    #[test]
    fn test_get_exe_path() {
        let result = get_exe_path();

        match result {
            Ok(path) => println!("Executable path: {}", path),
            Err(err) => println!("Error: {}", err),
        }

        let result1 = get_plugin_path();

        match result1 {
            Ok(path) => println!("Executable path: {}", path),
            Err(err) => println!("Error: {}", err),
        }
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            // 桌面平台特定的自动启动配置
            #[cfg(desktop)]
            {
                let _ = app.handle().plugin(tauri_plugin_autostart::init(
                    tauri_plugin_autostart::MacosLauncher::LaunchAgent,
                    Some(vec!["--flag1", "--flag2"]),
                ));
                // 获取自动启动管理器
                let autostart_manager = app.autolaunch();

                // 启用自动启动
                let _ = autostart_manager.enable();

                // 检查自动启动状态
                println!(
                    "registered for autostart? {}",
                    autostart_manager.is_enabled().unwrap()
                );

                // 禁用自动启动（如果需要）
                // let _ = autostart_manager.disable();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            my_custom_command, 
            greet, 
            get_exe_path,
            get_plugin_path]) // 注册命令
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
