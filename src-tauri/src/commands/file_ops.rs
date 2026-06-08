use std::fs;
use std::io::Write;
use tauri;

#[tauri::command]
pub fn save_svg(path: String, svg: String) -> Result<(), String> {
    let mut file = fs::File::create(&path)
        .map_err(|e| format!("Cannot create file: {}", e))?;
    file.write_all(svg.as_bytes())
        .map_err(|e| format!("Cannot write file: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn copy_svg_to_clipboard(
    app_handle: tauri::AppHandle,
    svg: String,
) -> Result<(), String> {
    use tauri_plugin_clipboard_manager::ClipboardExt;
    app_handle
        .clipboard()
        .write_text(svg)
        .map_err(|e| format!("Clipboard error: {}", e))?;
    Ok(())
}
