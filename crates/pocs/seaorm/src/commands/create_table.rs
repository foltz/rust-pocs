use sea_orm::{ConnectionTrait, DbBackend, DbConn, DbErr, EntityTrait, ExecResult, Schema};
use sea_orm::sea_query::{Table, TableCreateStatement};

pub async fn create_table<E>(
    conn: &DbConn,
    create: &TableCreateStatement,
    entity: E,
) -> Result<ExecResult, DbErr>
    where
        E: EntityTrait,
{
    let builder = conn.get_database_backend();
    let schema = Schema::new(builder);
    assert_eq!(
        builder.build(&schema.create_table_from_entity(entity)),
        builder.build(create)
    );

    create_table_without_asserts(conn, create).await
}

pub async fn create_table_without_asserts(
    db: &DbConn,
    create: &TableCreateStatement,
) -> Result<ExecResult, DbErr> {
    let builder = db.get_database_backend();

    // - TODO: setup Postgres handler here...

    if builder != DbBackend::Sqlite {
        let stmt = builder.build(
            Table::drop()
                .table(create.get_table_name().unwrap().clone())
                .if_exists()
                .cascade(),
        );
        db.execute(stmt).await?;
    }
    db.execute(builder.build(create)).await
}