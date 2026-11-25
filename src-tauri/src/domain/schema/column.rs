use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SqlTypeKind {
    VarChar,
    NVarChar,
    Int,
    BigInt,
    SmallInt,
    TinyInt,
    Decimal,
    Float,
    Real,
    Bit,
    DateTime,
    Date,
    UniqueIdentifier,
    VarBinary,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnSpec {
    pub raw_name: String,
    pub sql_type: SqlTypeKind,
    pub is_nullable: bool,
}