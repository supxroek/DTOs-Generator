// src-tauri/src/domain/codegen/dto_generator.rs
use serde::{Deserialize, Serialize};

use crate::domain::codegen::naming::to_pascal_case;
use crate::domain::codegen::type_mapping::{CSharpTarget, TypeMappingStrategy};
use crate::domain::schema::column::ColumnSpec;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodegenOptions {
    pub class_name: String,
    pub namespace: String,
    pub target: CSharpTarget,
    pub table_var_name: String,
    pub row_var_name: String,
}

pub fn generate_dto_class<M: TypeMappingStrategy>(
    columns: &[ColumnSpec],
    opts: &CodegenOptions,
    mapper: &M,
) -> String {
    let class_ident = to_pascal_case(&opts.class_name);
    let mut props = String::new();

    for col in columns {
        let prop_name = to_pascal_case(&col.raw_name);
        let cs_type = mapper.map_csharp_type(&col.sql_type, col.is_nullable, opts.target);
        props.push_str(&format!("        public {cs_type} {prop_name} {{ get; set; }}\n"));
    }

    match opts.target {
        CSharpTarget::NetModern => format!(
            "using System;\n\nnamespace {};\n\npublic class {class_ident}\n{{\n{props}}}\n",
            opts.namespace
        ),
        CSharpTarget::NetFramework4x => format!(
            "using System;\n\nnamespace {}\n{{\n    public class {class_ident}\n    {{\n{}    }}\n}}\n",
            opts.namespace,
            props.replace("        ", "        ")
        ),
    }
}