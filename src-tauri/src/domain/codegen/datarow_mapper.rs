// src-tauri/src/domain/codegen/datarow_mapper.rs
use crate::domain::codegen::naming::to_pascal_case;
use crate::domain::codegen::type_mapping::{CSharpTarget, TypeMappingStrategy};
use crate::domain::schema::column::ColumnSpec;
use crate::domain::codegen::dto_generator::CodegenOptions;

pub fn generate_datarow_loop<M: TypeMappingStrategy>(
    columns: &[ColumnSpec],
    opts: &CodegenOptions,
    mapper: &M,
) -> String {
    let class_ident = to_pascal_case(&opts.class_name);
    let list_ident = format!("{}List", to_pascal_case(&opts.class_name));
    let row_var = &opts.row_var_name;
    let table_var = &opts.table_var_name;

    let mut assignments = String::new();
    for col in columns {
        let prop_name = to_pascal_case(&col.raw_name);
        let col_accessor = format!(r#"{row_var}["{}"]"#, col.raw_name.replace('"', r#"\""#));
        let expr = mapper.map_datarow_expr(&col_accessor, &col.sql_type, col.is_nullable);
        assignments.push_str(&format!("        item.{prop_name} = {expr};\n"));
    }

    let nullable_note = match opts.target {
        CSharpTarget::NetModern => "// หมายเหตุ: หากเปิด Nullable Reference Types และใช้ string? ให้ปรับตามความเหมาะสม\n",
        CSharpTarget::NetFramework4x => "",
    };

    format!(
        "{nullable_note}var {list_ident} = new List<{class_ident}>();\n\nforeach (DataRow {row_var} in {table_var}.Rows)\n{{\n    var item = new {class_ident}();\n{assignments}    {list_ident}.Add(item);\n}}\n"
    )
}