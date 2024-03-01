use sea_orm::{error::*, sea_query, DatabaseConnection, DbConn, ExecResult};
use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Index, Table};

use crate::commands::create_table::create_table;
use crate::domain::bakery_chain::*;

pub async fn create_bakery_chain_tables(conn: &DatabaseConnection) -> Result<(), DbErr> {
    create_bakery_table(conn).await?;
    create_baker_table(conn).await?;
    create_customer_table(conn).await?;
    create_order_table(conn).await?;
    create_cake_table(conn).await?;
    create_cakes_bakers_table(conn).await?;
    create_lineitem_table(conn).await?;

    Ok(())
}

pub async fn create_bakery_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = Table::create()
        .table(bakery::Entity)
        .col(
            ColumnDef::new(bakery::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(bakery::Column::Name).string().not_null())
        .col(
            ColumnDef::new(bakery::Column::ProfitMargin)
                .double()
                .not_null(),
        )
        .to_owned();

    create_table(conn, &stmt, bakery::Entity).await
}

pub async fn create_baker_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = Table::create()
        .table(baker::Entity)
        .col(
            ColumnDef::new(baker::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(baker::Column::Name).string().not_null())
        .col(
            ColumnDef::new(baker::Column::ContactDetails)
                .json()
                .not_null(),
        )
        .col(ColumnDef::new(baker::Column::BakeryId).integer())
        .foreign_key(
            ForeignKey::create()
                .name("fk-baker-bakery_id")
                .from(baker::Entity, baker::Column::BakeryId)
                .to(bakery::Entity, bakery::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned();

    create_table(conn, &stmt, baker::Entity).await
}

pub async fn create_customer_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = Table::create()
        .table(customer::Entity)
        .col(
            ColumnDef::new(customer::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(customer::Column::Name).string().not_null())
        .col(ColumnDef::new(customer::Column::Notes).text())
        .to_owned();

    create_table(conn, &stmt, customer::Entity).await
}

pub async fn create_order_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = Table::create()
        .table(order::Entity)
        .col(
            ColumnDef::new(order::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(order::Column::Total)
                .decimal_len(19, 4)
                .not_null(),
        )
        .col(ColumnDef::new(order::Column::BakeryId).integer().not_null())
        .col(
            ColumnDef::new(order::Column::CustomerId)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(order::Column::PlacedAt)
                .date_time()
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .name("fk-order-bakery_id")
                .from(order::Entity, order::Column::BakeryId)
                .to(bakery::Entity, bakery::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .name("fk-order-customer_id")
                .from(order::Entity, order::Column::CustomerId)
                .to(customer::Entity, customer::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned();

    create_table(conn, &stmt, order::Entity).await
}

pub async fn create_lineitem_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = Table::create()
        .table(lineitem::Entity)
        .col(
            ColumnDef::new(lineitem::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(lineitem::Column::Price)
                .decimal_len(19, 4)
                .not_null(),
        )
        .col(
            ColumnDef::new(lineitem::Column::Quantity)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(lineitem::Column::OrderId)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(lineitem::Column::CakeId)
                .integer()
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .name("fk-lineitem-order_id")
                .from(lineitem::Entity, lineitem::Column::OrderId)
                .to(order::Entity, order::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .name("fk-lineitem-cake_id")
                .from(lineitem::Entity, lineitem::Column::CakeId)
                .to(cake::Entity, cake::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned();

    create_table(conn, &stmt, lineitem::Entity).await
}

pub async fn create_cakes_bakers_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = Table::create()
        .table(cakes_bakers::Entity)
        .col(
            ColumnDef::new(cakes_bakers::Column::CakeId)
                .integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(cakes_bakers::Column::BakerId)
                .integer()
                .not_null(),
        )
        .primary_key(
            Index::create()
                .name("pk-cakes_bakers")
                .col(cakes_bakers::Column::CakeId)
                .col(cakes_bakers::Column::BakerId),
        )
        .foreign_key(
            ForeignKey::create()
                .name("fk-cakes_bakers-cake_id")
                .from(cakes_bakers::Entity, cakes_bakers::Column::CakeId)
                .to(cake::Entity, cake::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .name("fk-cakes_bakers-baker_id")
                .from(cakes_bakers::Entity, cakes_bakers::Column::BakerId)
                .to(baker::Entity, baker::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .to_owned();

    create_table(conn, &stmt, cakes_bakers::Entity).await
}

pub async fn create_cake_table(conn: &DbConn) -> Result<ExecResult, DbErr> {
    let stmt = Table::create()
        .table(cake::Entity)
        .col(
            ColumnDef::new(cake::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(cake::Column::Name).string().not_null())
        .col(
            ColumnDef::new(cake::Column::Price)
                .decimal_len(19, 4)
                .not_null(),
        )
        .col(ColumnDef::new(cake::Column::BakeryId).integer())
        .foreign_key(
            ForeignKey::create()
                .name("fk-cake-bakery_id")
                .from(cake::Entity, cake::Column::BakeryId)
                .to(bakery::Entity, bakery::Column::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade),
        )
        .col(
            ColumnDef::new(cake::Column::GlutenFree)
                .boolean()
                .not_null(),
        )
        .col(ColumnDef::new(cake::Column::Serial).uuid().not_null())
        .to_owned();

    create_table(conn, &stmt, cake::Entity).await
}
