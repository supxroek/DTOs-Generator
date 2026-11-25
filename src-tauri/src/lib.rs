pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interface;

use interface::{
    cmd_describe_query, cmd_describe_table, cmd_generate_dto, cmd_list_databases, cmd_list_tables,
    cmd_save_cs_file, cmd_test_connection,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            cmd_test_connection,
            cmd_list_databases,
            cmd_list_tables,
            cmd_describe_table,
            cmd_describe_query,
            cmd_generate_dto,
            cmd_save_cs_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}