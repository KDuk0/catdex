use actix_files::Files;
use actix_web::{web, App, HttpServer, HttpResponse};

use handlebars::Handlebars;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = serde_json::json!({
        "project_name": "PepeCatdex",
        "cats": [
            {
                "name": "Pepe with a cat",
                "image_path": "/static/image/1.jpg"
            },
            {
                "name": "Pepe with another cat",
                "image_path": "/static/image/2.jpg"
            },
            {
                "name": "Pepe with a cat perhaps?",
                "image_path": "/static/image/3.jpg"
            },
        ]

    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("Listening on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(
                Files::new("/static", "static")
                    .show_files_listing(),
            )
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}