#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
async fn hello_world_command(_app: tauri::AppHandle) -> Result<String, String> {
    println!("I was invoked from JS!");
    Ok("Hello world from Tauri!".into())
}

#[tauri::command]
async fn number_exponent_3(n: i32) -> i32 {
    n * n * n
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            hello_world_command,
            number_exponent_3
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
