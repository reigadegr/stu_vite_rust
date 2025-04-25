use anyhow::{Result, anyhow};
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;

const MYSQL_URI: &str = "mysql://root:1234@127.0.0.1:3306/bs_desktop?characterEncoding=utf-8&serverTimezone=UTC&useSSL=false&allowPublicKeyRetrieval=true";
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();
pub async fn init_db_conn() {
    DB.get_or_init(|| async { Database::connect(MYSQL_URI).await.unwrap_or_default() })
        .await;
}

#[inline]
pub fn get_db_con<'a>() -> Result<&'a DatabaseConnection> {
    DB.get().ok_or_else(|| anyhow!("DB not found"))
}
