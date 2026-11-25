use serde::{Deserialize, Serialize};
use thiserror::Error;
use tiberius::{AuthMethod, Client, Config, EncryptionLevel};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub trust_cert: bool,
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("TCP connection failed: {0}")]
    Tcp(#[from] std::io::Error),
    #[error("SQL Server TDS error: {0}")]
    Tiberius(#[from] tiberius::error::Error),
}

pub type DbResult<T> = Result<T, DbError>;
pub type SqlClient = Client<Compat<TcpStream>>;

pub async fn connect_sql_server(cfg: &ConnectionConfig) -> DbResult<SqlClient> {
    let mut config = Config::new();
    config.host(&cfg.host);
    config.port(cfg.port);
    config.database(&cfg.database);
    config.authentication(AuthMethod::sql_server(&cfg.username, &cfg.password));
    config.encryption(EncryptionLevel::NotSupported);

    if cfg.trust_cert {
        config.trust_cert();
    }

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp.compat_write()).await?;
    Ok(client)
}