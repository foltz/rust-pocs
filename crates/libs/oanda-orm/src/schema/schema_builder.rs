use sea_orm::{error::*, DatabaseConnection};
use crate::schema::table_defs::*;

pub async fn create_oanda_schema(conn: &DatabaseConnection) -> Result<(), DbErr> {

    // create_candlestick_data(conn).await?;
    // create_candlestick_query_log(conn).await?;
    // 
    // create_order_book_data(conn).await?;
    // create_order_book_query_log(conn).await?;
    // 
    // create_position_book_data(conn).await?;
    // create_position_book_query_log(conn).await?;

    // create_pricing_aggregate_1s(conn).await?;
    create_pricing_stream_analysis(conn).await?;
    
    // create_pricing_stream_data(conn).await?;
    // create_pricing_stream_data_log(conn).await?;
    // 
    // create_pricing_stream_session(conn).await?;
    // create_pricing_stream_session_log(conn).await?;

    Ok(())
}

pub async fn update_oanda_schema(conn: &DatabaseConnection) -> Result<(), DbErr> {

    // create_candlestick_data(conn).await?;
    // create_candlestick_query_log(conn).await?;
    //
    // create_order_book_data(conn).await?;
    // create_order_book_query_log(conn).await?;
    //
    // create_position_book_data(conn).await?;
    // create_position_book_query_log(conn).await?;
    //
    // create_pricing_stream_data(conn).await?;
    // create_pricing_stream_data_log(conn).await?;

    // create_pricing_stream_session(conn).await?;
    // create_pricing_stream_session_log(conn).await?;

    Ok(())
}