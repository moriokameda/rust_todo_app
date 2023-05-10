use actix_web::{App, get, HttpResponse, HttpServer, ResponseError};
use askama::Template;
use thiserror::Error;

struct TodoEntity {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntity>,
}

#[derive(Error, Debug)]
enum MyError {
    #[error("failed to render HTML")]
    AskamaError(#[from] askama::Error)
}

impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();
    entries.push(TodoEntity {
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(TodoEntity {
        id: 2,
        text: "Second entry".to_string(),
    });
    let html = IndexTemplate { entries };

    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index)).bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
