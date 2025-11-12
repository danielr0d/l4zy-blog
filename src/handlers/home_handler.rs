use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::{fs, io::Error};
use ignore::WalkBuilder;

fn find_all_frontmatter() -> Result<Vec<Frontmatter>, std::io::Error> {
    let mut t = ignore::types::TypesBuilder::new();
    t.add_defaults();
    let toml = match t.select("toml").build() {
        Ok(t)=> t,
        Err(e)=> {
            println!{"{:}", e};
            return Err(Error::new(std::io::ErrorKind::Other,
            "could not build toml file type matcher"))
        }
    };

    let file_walker = WalkBuilder::new("./posts").types(toml).build();

    let mut frontmatters = Vec::new();
    for frontmatter in file_walker {
        match frontmatter {
            Ok(fm) => {
                if fm.path().is_file() {
                    let fm_content = fs::read_to_string(fm.path())?;
                   let frontmatter: Frontmatter = toml::from_str(&fm_content)?;

                   frontmatters.push(frontmatter);
                }
            }
        Err(e) => {
            println!("{:}", e);
            return Err(Error::new(std::io::ErrorKind::NotFound, "could not locate frontmatter"))
        }
      }
    }
    Ok(frontmatters)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Frontmatter {
    title: String,
    file_name: String,
    description: String,
    posted: String,
    tags: Vec<String>,
    author: String,
    estimated_reading_time: u32,
    order: u32,
}

#[get("/")]
pub async fn index(templates: web::Data<tera::Tera>) -> impl Responder {
    let mut context = tera::Context::new();

    let mut frontmatters = match find_all_frontmatters() {
        Ok(fm) => fm,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Me descula por ser burro</p>");
        }
    };

    frontmatters.sort_by(|a, b| b.order.cmp(&a.oder));

    context.insert("posts", &frontmatters);

    match templates.render("home.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Pegou fogo na caixa dagua</p>")
        }
    }
}

