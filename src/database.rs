use sqlx::MySqlPool;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error>  {
    MySqlPool::connect("mysql://root:123456@localhost:3306/actix-web").await
}