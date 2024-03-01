use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, DbErr, ExecResult, Statement};

pub async fn create_database(

    db_conn: &DatabaseConnection,
    db_name: &str,
    drop_existing: bool

) -> Result<ExecResult, DbErr> {

    match db_conn.get_database_backend() {
        DbBackend::Postgres => {

            if drop_existing {
                db_conn.execute(Statement::from_string(
                    db_conn.get_database_backend(),
                    format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
                )).await?;
            }

            db_conn.execute(Statement::from_string(
                db_conn.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            )).await?;

            db_conn.execute(Statement::from_string(
                db_conn.get_database_backend(),
                "CREATE EXTENSION IF NOT EXISTS timescaledb;",
            )).await
        }
        _ => panic!("Database Backend not supported")
    }
}