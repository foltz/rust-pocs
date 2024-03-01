use sea_orm::{Database, DbConn, DbErr};

pub async fn open_connection(db_host: &String, db_name: &String, db_user: &String, db_pass: &String, db_ssl: &String) -> Result<DbConn, DbErr> {

    // let db_url: &str = "postgres://postgres:password@localhost:5432";
    let db_url= format!("postgres://{db_user}:{db_pass}@{db_host}/{db_name}?sslmode={db_ssl}");
    Database::connect(db_url).await
}