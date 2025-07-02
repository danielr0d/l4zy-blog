use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, middleware};
use tera::Terra;

pub fn start_blog(listener: TcpListener) -> Result <Server, std::io::Error> {
    let server = HttpServer::new(move ||{
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(HttpResponse::Ok))
    })
    .listen(listerner)?
    .run();

    Ok(srv)
}
