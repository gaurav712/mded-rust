mod markdown;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn parse_markdown(markdown: String) -> Result<String, String> {
    match markdown::process_markdown_parallel(&markdown, None) {
        html => Ok(html),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, parse_markdown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
