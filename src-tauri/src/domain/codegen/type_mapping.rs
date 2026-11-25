use serde::{Deserialize, Serialize};
use crate::domain::schema::column::SqlTypeKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CSharpTarget {
    NetFramework4x,
    NetModern, // .NET 8+
}

pub trait TypeMappingStrategy {
    fn map_csharp_type(&self, sql_type: &SqlTypeKind, is_nullable: bool, target: CSharpTarget) -> String;
    fn map_datarow_expr(&self, prop_accessor: &str, sql_type: &SqlTypeKind, is_nullable: bool) -> String;
}

pub struct WinFormsDefaultMapper;

impl WinFormsDefaultMapper {
    fn base_csharp_type(sql_type: &SqlTypeKind) -> (&'static str, bool) {
        // คืนค่า (ชื่อ C# type พื้นฐาน, เป็น Value Type หรือไม่)
        match sql_type {
            SqlTypeKind::VarChar | SqlTypeKind::NVarChar => ("string", false),
            SqlTypeKind::Int => ("int", true),
            SqlTypeKind::BigInt => ("long", true),
            SqlTypeKind::SmallInt => ("short", true),
            SqlTypeKind::TinyInt => ("byte", true),
            SqlTypeKind::Decimal => ("decimal", true),
            SqlTypeKind::Float => ("double", true),
            SqlTypeKind::Real => ("float", true),
            SqlTypeKind::Bit => ("bool", true),
            SqlTypeKind::DateTime | SqlTypeKind::Date => ("DateTime", true),
            SqlTypeKind::UniqueIdentifier => ("Guid", true),
            SqlTypeKind::VarBinary => ("byte[]", false),
            SqlTypeKind::Other(_) => ("object", false),
        }
    }
}

impl TypeMappingStrategy for WinFormsDefaultMapper {
    fn map_csharp_type(&self, sql_type: &SqlTypeKind, is_nullable: bool, target: CSharpTarget) -> String {
        let (base, is_value_type) = Self::base_csharp_type(sql_type);

        match (is_nullable, is_value_type, target) {
            (true, true, _) => format!("{base}?"),
            (true, false, CSharpTarget::NetModern) => format!("{base}?"),
            _ => base.to_string(),
        }
    }

    fn map_datarow_expr(&self, col_expr: &str, sql_type: &SqlTypeKind, is_nullable: bool) -> String {
        let (base, is_value_type) = Self::base_csharp_type(sql_type);

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

        if !is_nullable {
            return non_null_expr;
        }

        // รองรับ C# 7.3 (.NET Framework): Value Type ต้อง cast เป็น (T?)null
        let null_literal = if is_value_type {
            format!("({base}?)null")
        } else {
            "null".to_string()
        };

        format!("{col_expr} == DBNull.Value ? {null_literal} : {non_null_expr}")
    }
}