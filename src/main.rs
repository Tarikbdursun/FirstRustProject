use actix_web::{App, HttpServer, web::Data};

mod routes;
use routes::*;

mod database;
use database::*;

#[tokio::main] 
async fn main() ->std::io::Result<()> 
{
    let database = database_connection()
        .await
        .expect("Failed to connect to database");

    println!("Database connection established");

    let server=HttpServer::new(move ||
        { 
            App::new()
                .app_data(Data::new(database.clone()))
                .service(home)
                .service(hello_user)
                .service(create_new_user)
                .service(create_new_todo)
                .service(get_all_todos)
        })
        .bind(("127.0.0.1",8000))?
        .run();

    println!("Server Running At 127.0.0.1:8000");
    server.await
}
