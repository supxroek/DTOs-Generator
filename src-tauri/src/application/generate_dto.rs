use serde::Serialize;

use crate::domain::codegen::datarow_mapper::generate_datarow_loop;
use crate::domain::codegen::dto_generator::{generate_dto_class, CodegenOptions};
use crate::domain::codegen::type_mapping::WinFormsDefaultMapper;
use crate::domain::schema::column::ColumnSpec;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedCodeOutput {
    pub dto_code: String,
    pub mapper_code: String,
}

pub fn run_generate_dto(columns: &[ColumnSpec], options: &CodegenOptions) -> GeneratedCodeOutput {
    let mapper = WinFormsDefaultMapper;
    let dto_code = generate_dto_class(columns, options, &mapper);
    let mapper_code = generate_datarow_loop(columns, options, &mapper);

    GeneratedCodeOutput {
        dto_code,
        mapper_code,
    }
}