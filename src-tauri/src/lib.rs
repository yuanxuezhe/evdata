#[cfg_attr(mobile, tauri::mobile_entry_point)]

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

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![my_custom_command, greet]) // 注册命令
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}