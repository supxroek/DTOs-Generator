use serde::{Deserialize, Serialize};

use crate::application::generate_dto::{run_generate_dto, GeneratedCodeOutput};
use crate::domain::codegen::dto_generator::CodegenOptions;
use crate::domain::schema::column::ColumnSpec;
use crate::infrastructure::fs::cs_writer::{write_cs_file, SavedFileInfo};
use crate::infrastructure::mssql::client::ConnectionConfig;
use crate::infrastructure::mssql::schema_reader::{
    TableInfo, describe_query, describe_table, list_databases, list_tables,
};

#[derive(Debug, Serialize)]
pub struct CommandError {
    pub message: String,
}

impl<E: std::fmt::Display> From<E> for CommandError {
    fn from(err: E) -> Self {
        Self {
            message: err.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveCsFileRequest {
    pub folder_path: String,
    pub file_name: String,
    pub content: String,
    pub overwrite: bool,
}

#[tauri::command]
pub async fn cmd_test_connection(config: ConnectionConfig) -> Result<bool, CommandError> {
    let _ = list_tables(&config).await?;
    Ok(true)
}

#[tauri::command]
pub async fn cmd_list_tables(config: ConnectionConfig) -> Result<Vec<TableInfo>, CommandError> {
    list_tables(&config).await.map_err(Into::into)
}

#[tauri::command]
pub async fn cmd_describe_table(
    config: ConnectionConfig,
    schema: String,
    table_name: String,
) -> Result<Vec<ColumnSpec>, CommandError> {
    describe_table(&config, &schema, &table_name)
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn cmd_describe_query(
    config: ConnectionConfig,
    sql: String,
) -> Result<Vec<ColumnSpec>, CommandError> {
    describe_query(&config, &sql).await.map_err(Into::into)
}

#[tauri::command]
pub fn cmd_generate_dto(
    columns: Vec<ColumnSpec>,
    options: CodegenOptions,
) -> Result<GeneratedCodeOutput, CommandError> {
    Ok(run_generate_dto(&columns, &options))
}

#[tauri::command]
pub fn cmd_save_cs_file(req: SaveCsFileRequest) -> Result<SavedFileInfo, CommandError> {
    write_cs_file(&req.folder_path, &req.file_name, &req.content, req.overwrite)
        .map_err(Into::into)
}

#[tauri::command]
pub async fn cmd_list_databases(config: ConnectionConfig) -> Result<Vec<String>, CommandError> {
    list_databases(&config).await.map_err(Into::into)
}