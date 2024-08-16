use actix_web::{http::Error, App, HttpServer};

mod routes;
use routes::*;

mod database;
use database::*;

#[tokio::main] //Result<Pool<MySql>, Error>
async fn main() ->std::io::Result<()> 
{
    let database = database_connection()
        .await
        .expect("Failed to connect to database");

    println!("Database connection established");

    let server=HttpServer::new(move || 
    App::new()
        .app_data(database.clone())
        .service(home)
        .service(hello_user)
        .service(create_new_user))
        .bind(("127.0.0.1",8000))?
        .run();

    println!("Server Running At 127.0.0.1:8000");
    server.await
}
