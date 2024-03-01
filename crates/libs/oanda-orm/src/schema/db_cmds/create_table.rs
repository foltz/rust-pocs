use sea_orm::{ColumnTrait, ConnectionTrait, DbBackend, DbConn, DbErr, EntityTrait, ExecResult, Schema};
use sea_orm::sea_query::{ColumnDef, IndexCreateStatement, IntoIden, IntoIndexColumn, Table, TableCreateStatement};

pub async fn create_table<E>(
    db_conn: &DbConn,
    create_stmt: &TableCreateStatement,
    entity: E,
) -> Result<ExecResult, DbErr>
    where
        E: EntityTrait,
{
    let builder = db_conn.get_database_backend();
    let schema = Schema::new(builder);
    assert_eq!(
        builder.build(&schema.create_table_from_entity(entity)),
        builder.build(create_stmt)
    );

    create_table_without_asserts(db_conn, create_stmt).await
}

pub async fn create_hypertable<E, C>(
    db_conn: &DbConn,
    entity: E,
    column: C
) -> Result<ExecResult, DbErr>
    where
        E: EntityTrait,
        C: ColumnTrait,
{
    let table_name = entity.table_name();
    let col_name = ColumnDef::new(column).get_column_name();

    let stmt = format!("SELECT create_hypertable('{table_name}', '{col_name}');");

    db_conn.execute_unprepared(&stmt).await
}

pub async fn create_table_without_asserts(
    db_conn: &DbConn,
    create_stmt: &TableCreateStatement,
) -> Result<ExecResult, DbErr> {
    
    let builder = db_conn.get_database_backend();

    // - TODO: setup Postgres handler here...

    if builder != DbBackend::Sqlite {
        let stmt = builder.build(
            Table::drop()
                .table(create_stmt.get_table_name().unwrap().clone())
                .if_exists()
                .cascade(),
        );
        db_conn.execute(stmt).await?;
    }
    db_conn.execute(builder.build(create_stmt)).await
}

pub async fn create_index(
    db_conn: &DbConn,
    create_stmt: &IndexCreateStatement,
) -> Result<ExecResult, DbErr> {

    let builder = db_conn.get_database_backend();
    db_conn.execute(builder.build(create_stmt)).await
}