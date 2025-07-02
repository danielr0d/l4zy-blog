use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, middleware};
use tera::Terra;

#[macro_use]
extern create lazy_static;

lazy_static{
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process:exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}


pub fn start_blog(listener: TcpListener) -> Result <Server, std::io::Error> {
    let server = HttpServer::new(move ||{
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health", web::get().to(HttpResponse::Ok))
    })
    .listen(listerner)?
    .run();

    Ok(srv)

