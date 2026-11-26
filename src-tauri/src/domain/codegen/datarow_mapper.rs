use crate::domain::codegen::dto_generator::CodegenOptions;
use crate::domain::codegen::naming::to_pascal_case;
use crate::domain::codegen::type_mapping::TypeMappingStrategy;
use crate::domain::schema::column::ColumnSpec;

pub fn generate_datarow_loop<M: TypeMappingStrategy>(
    columns: &[ColumnSpec],
    opts: &CodegenOptions,
    mapper: &M,
) -> String {
    let class_ident = to_pascal_case(&opts.class_name);
    let row_var = if opts.row_var_name.trim().is_empty() {
        "row"
    } else {
        opts.row_var_name.trim()
    };
    let table_var = if opts.table_var_name.trim().is_empty() {
        "result.dtData"
    } else {
        opts.table_var_name.trim()
    };

    let mut prop_inits = Vec::with_capacity(columns.len());
    for col in columns {
        let prop_name = to_pascal_case(&col.raw_name);
        let col_accessor = format!(r#"{row_var}["{}"]"#, col.raw_name.replace('"', r#"\""#));
        let expr = mapper.map_datarow_expr(&col_accessor, &col.sql_type, col.is_nullable);
        prop_inits.push(format!("        {prop_name} = {expr}"));
    }

    let body = prop_inits.join(",\n");

    format!(
        "var list = new List<{class_ident}>();\n\nif ({table_var} == null || {table_var}.Rows.Count == 0)\nreturn list;\n\n// Mapping\nforeach (DataRow {row_var} in {table_var}.Rows)\n{{\n\tlist.Add(new {class_ident}\n\t{{\n{body}\n\t}});\n}}\n\nreturn list;\n"
    )
}