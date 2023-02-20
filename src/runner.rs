use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use crate::domain::user::service;
use crate::application::handler;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;


pub fn run_http_server(listener: TcpListener, user_service: service::Service) -> Result<Server, std::io::Error> {
    let data_user_service = Data::new(user_service);

    let server = HttpServer::new(move|| {
        App::new()
        .wrap(TracingLogger::default())
        .app_data(web::JsonConfig::default().limit(4096))
        .app_data(data_user_service.clone())
        .route("/user",web::post().to(handler::create_user))
        .route("/user", web::get().to(handler::list_users))
        .route("/user/{id}", web::get().to(handler::get_user))
        .route("/user/{id}", web::delete().to(handler::delete_user))
    })
    .listen(listener)?
    .run();
    Ok(server)
}