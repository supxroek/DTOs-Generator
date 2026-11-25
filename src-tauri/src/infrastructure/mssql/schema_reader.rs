use serde::{Deserialize, Serialize};
use tiberius::Query;

use crate::domain::schema::column::{ColumnSpec, SqlTypeKind};
use crate::infrastructure::mssql::client::{connect_sql_server, ConnectionConfig, DbResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableInfo {
    pub schema: String,
    pub name: String,
}

pub fn map_sql_type_name(raw_type: &str) -> SqlTypeKind {
    // ตัดส่วนความยาว/precision เช่น varchar(100) หรือ decimal(18,2) ออกก่อน
    let base = raw_type
        .split('(')
        .next()
        .unwrap_or(raw_type)
        .trim()
        .to_lowercase();

    match base.as_str() {
        "varchar" | "char" | "text" => SqlTypeKind::VarChar,
        "nvarchar" | "nchar" | "ntext" => SqlTypeKind::NVarChar,
        "int" => SqlTypeKind::Int,
        "bigint" => SqlTypeKind::BigInt,
        "smallint" => SqlTypeKind::SmallInt,
        "tinyint" => SqlTypeKind::TinyInt,
        "decimal" | "numeric" | "money" | "smallmoney" => SqlTypeKind::Decimal,
        "float" => SqlTypeKind::Float,
        "real" => SqlTypeKind::Real,
        "bit" => SqlTypeKind::Bit,
        "datetime" | "datetime2" | "smalldatetime" => SqlTypeKind::DateTime,
        "date" => SqlTypeKind::Date,
        "uniqueidentifier" => SqlTypeKind::UniqueIdentifier,
        "varbinary" | "binary" | "image" => SqlTypeKind::VarBinary,
        other => SqlTypeKind::Other(other.to_string()),
    }
}

pub async fn list_tables(cfg: &ConnectionConfig) -> DbResult<Vec<TableInfo>> {
    let mut client = connect_sql_server(cfg).await?;
    let sql = r#"
        SELECT TABLE_SCHEMA, TABLE_NAME
        FROM INFORMATION_SCHEMA.TABLES
        WHERE TABLE_TYPE IN ('BASE TABLE', 'VIEW')
        ORDER BY TABLE_SCHEMA, TABLE_NAME
    "#;

    let stream = client.simple_query(sql).await?;
    let rows = stream.into_first_result().await?;

    let tables = rows
        .into_iter()
        .filter_map(|row| {
            let schema = row.get::<&str, _>(0)?.to_string();
            let name = row.get::<&str, _>(1)?.to_string();
            Some(TableInfo { schema, name })
        })
        .collect();

    Ok(tables)
}

pub async fn describe_query(cfg: &ConnectionConfig, tsql: &str) -> DbResult<Vec<ColumnSpec>> {
    let mut client = connect_sql_server(cfg).await?;

    let mut query = Query::new("EXEC sys.sp_describe_first_result_set @tsql = @P1");
    let prepared_sql = normalize_tsql_for_describe(tsql);
    query.bind(&prepared_sql);

    let stream = query.query(&mut client).await?;
    let rows = stream.into_first_result().await?;

    let mut cols = Vec::new();
    for (idx, row) in rows.into_iter().enumerate() {
        let raw_name = row
            .get::<&str, _>("name")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| format!("Column{}", idx + 1));

        let type_name = row
            .get::<&str, _>("system_type_name")
            .unwrap_or("sql_variant");

        let is_nullable = row.get::<bool, _>("is_nullable").unwrap_or(true);

        cols.push(ColumnSpec {
            raw_name,
            sql_type: map_sql_type_name(type_name),
            is_nullable,
        });
    }

    Ok(cols)
}

pub async fn describe_table(
    cfg: &ConnectionConfig,
    schema: &str,
    table_name: &str,
) -> DbResult<Vec<ColumnSpec>> {
    let mut client = connect_sql_server(cfg).await?;

    let sql = r#"
        SELECT
            COLUMN_NAME,
            DATA_TYPE,
            IS_NULLABLE
        FROM INFORMATION_SCHEMA.COLUMNS
        WHERE TABLE_SCHEMA = @P1
          AND TABLE_NAME = @P2
        ORDER BY ORDINAL_POSITION
    "#;

    let mut query = Query::new(sql);
    query.bind(schema);
    query.bind(table_name);

    let stream = query.query(&mut client).await?;
    let rows = stream.into_first_result().await?;

    let mut cols = Vec::with_capacity(rows.len());
    for (idx, row) in rows.into_iter().enumerate() {
        let raw_name = row
            .get::<&str, _>(0)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| format!("Column{}", idx + 1));

        let data_type = row.get::<&str, _>(1).unwrap_or("sql_variant");

        // ใน INFORMATION_SCHEMA.COLUMNS ค่า IS_NULLABLE มักเป็น "YES" หรือ "NO"
        let is_nullable = row
            .get::<&str, _>(2)
            .map(|v| v.trim().eq_ignore_ascii_case("YES"))
            .unwrap_or(true);

        cols.push(ColumnSpec {
            raw_name,
            sql_type: map_sql_type_name(data_type),
            is_nullable,
        });
    }

    Ok(cols)
}

pub async fn list_databases(cfg: &ConnectionConfig) -> DbResult<Vec<String>> {
    let mut client = connect_sql_server(cfg).await?;
    let sql = r#"
        SELECT name
        FROM sys.databases
        WHERE state_desc = 'ONLINE'
          AND HAS_DBACCESS(name) = 1
        ORDER BY name
    "#;

    let stream = client.simple_query(sql).await?;
    let rows = stream.into_first_result().await?;

    let dbs = rows
        .into_iter()
        .filter_map(|row| row.get::<&str, _>(0).map(|s| s.to_string()))
        .collect();

    Ok(dbs)
}

fn normalize_tsql_for_describe(tsql: &str) -> String {
    // กรณีพบ {whereClause} แบบ case-insensitive ให้แทนที่ด้วยช่องว่างเพื่อไม่ให้เกิด syntax error ตอน describe
    let mut result = tsql.to_string();
    let lower = result.to_lowercase();
    if let Some(start) = lower.find("{whereclause}") {
        result.replace_range(start..start + "{whereclause}".len(), " ");
    }
    result
}