use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, DbErr, ExecResult, Statement};

pub async fn create_database(

    conn: &DatabaseConnection,
    name: &str,
    drop_existing: bool

) -> Result<ExecResult, DbErr> {

    match conn.get_database_backend() {
        DbBackend::Postgres => {

            if drop_existing {
                conn.execute(Statement::from_string(
                    conn.get_database_backend(),
                    format!("DROP DATABASE IF EXISTS \"{}\";", name),
                )).await?;
            }

            conn.execute(Statement::from_string(
                conn.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", name),
            )).await?;

            conn.execute(Statement::from_string(
                conn.get_database_backend(),
                "CREATE EXTENSION IF NOT EXISTS timescaledb;",
            )).await
        }
        _ => panic!("Database Backend not supported")
    }
}