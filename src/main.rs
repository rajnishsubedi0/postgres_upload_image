use actix_multipart::Multipart;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures_util::StreamExt;
use std::{env, u8};
use tokio_postgres::{Client, NoTls};

async fn upload_image(mut payload: Multipart) -> impl Responder {
    let client = connect_db().await;
    while let Some(field) = payload.next().await {
        let mut field = field.unwrap();
        // let content_disposition = field.content_disposition().unwrap();
        // let filename = content_disposition.get_filename().unwrap_or("unknown").to_string();

        let mut image_data = Vec::new();
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            image_data.extend_from_slice(&chunk);
        }

        match client
            .execute("INSERT INTO images (data) VALUES ($1)", &[&image_data])
            .await
        {
            Ok(_) => return HttpResponse::Ok().body("Image added successfully"),
            Err(_) => return HttpResponse::InternalServerError().body("Failed to image"),
        };
    }

    HttpResponse::Ok().body("Image added successfully")
}
async fn get_image() -> impl Responder {
    let client = connect_db().await;
    let rows = client.query("SELECT data FROM images", &[]).await.unwrap();
    let a = &rows[0];
    let image_data: Vec<u8> = a.get("data");

    HttpResponse::Ok()
        .content_type("image/jpeg")
        .append_header((
            "Content-Disposition",
            format!("inline; filename=\"{}\"", "hola"),
        ))
        .body(image_data)
}

// Function to establish PostgreSQL connection
async fn connect_db() -> Client {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to the database");

    // Spawn a new async task to handle the database connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    client
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload_image))
            .route("/upload", web::get().to(get_image))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
