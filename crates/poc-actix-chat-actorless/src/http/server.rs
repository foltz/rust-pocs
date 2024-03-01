use actix_web::{App, HttpServer, middleware, web};
use actix_web::dev::Server;
use crate::app::AppContext;
use crate::http::routes;

pub fn run_http_server(
    app_ctx: AppContext,
    port: u16,
) -> Server { // - was: std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_ctx.clone()))

            // WebSocket UI HTML file
            .service(routes::get_index)

            // websocket routes
            .service(routes::get_ws)

            // enable logger
            .wrap(middleware::Logger::default())
    })
        .workers(2)
        .bind(("0.0.0.0", port)).unwrap() //?
        .run()
}