// src-tauri/src/domain/codegen/type_mapping.rs
use crate::domain::schema::column::SqlTypeKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CSharpTarget {
    NetFramework4x,
    NetModern,
}

pub trait TypeMappingStrategy {
    fn map_csharp_type(&self, sql_type: &SqlTypeKind, is_nullable: bool, target: CSharpTarget) -> String;
    fn map_datarow_expr(&self, prop_accessor: &str, sql_type: &SqlTypeKind, is_nullable: bool) -> String;
}

pub struct WinFormsDefaultMapper;

impl TypeMappingStrategy for WinFormsDefaultMapper {
    fn map_csharp_type(&self, sql_type: &SqlTypeKind, is_nullable: bool, target: CSharpTarget) -> String {
        let base = match sql_type {
            SqlTypeKind::VarChar | SqlTypeKind::NVarChar => "string",
            SqlTypeKind::Int => "int",
            SqlTypeKind::BigInt => "long",
            SqlTypeKind::SmallInt => "short",
            SqlTypeKind::TinyInt => "byte",
            SqlTypeKind::Decimal => "decimal",
            SqlTypeKind::Float => "double",
            SqlTypeKind::Real => "float",
            SqlTypeKind::Bit => "bool",
            SqlTypeKind::DateTime | SqlTypeKind::Date => "DateTime",
            SqlTypeKind::UniqueIdentifier => "Guid",
            SqlTypeKind::VarBinary => "byte[]",
            SqlTypeKind::Other(_) => "object",
        };

        let is_reference_type = matches!(
            sql_type,
            SqlTypeKind::VarChar | SqlTypeKind::NVarChar | SqlTypeKind::VarBinary | SqlTypeKind::Other(_)
        );

        match (is_nullable, is_reference_type, target) {
            (true, false, _) => format!("{base}?"),
            (true, true, CSharpTarget::NetModern) => format!("{base}?"),
            _ => base.to_string(),
        }
    }

    fn map_datarow_expr(&self, col_expr: &str, sql_type: &SqlTypeKind, is_nullable: bool) -> String {
        let non_null_expr = match sql_type {
            SqlTypeKind::VarChar | SqlTypeKind::NVarChar => format!("{col_expr}.ToString()"),
            SqlTypeKind::Int => format!("Convert.ToInt32({col_expr})"),
            SqlTypeKind::BigInt => format!("Convert.ToInt64({col_expr})"),
            SqlTypeKind::SmallInt => format!("Convert.ToInt16({col_expr})"),
            SqlTypeKind::TinyInt => format!("Convert.ToByte({col_expr})"),
            SqlTypeKind::Decimal => format!("Convert.ToDecimal({col_expr})"),
            SqlTypeKind::Float => format!("Convert.ToDouble({col_expr})"),
            SqlTypeKind::Real => format!("Convert.ToSingle({col_expr})"),
            SqlTypeKind::Bit => format!("Convert.ToBoolean({col_expr})"),
            SqlTypeKind::DateTime | SqlTypeKind::Date => format!("Convert.ToDateTime({col_expr})"),
            SqlTypeKind::UniqueIdentifier => format!("(Guid){col_expr}"),
            SqlTypeKind::VarBinary => format!("(byte[]){col_expr}"),
            SqlTypeKind::Other(_) => col_expr.to_string(),
        };

        if is_nullable {
            format!("{col_expr} == DBNull.Value ? null : {non_null_expr}")
        } else {
            non_null_expr
        }
    }
}