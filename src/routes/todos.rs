use actix_web::web::Json;
use actix_web::HttpResponse;
use actix_web::{web::Data, get,post,Responder};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::Error;
use sqlx::MySqlPool;

#[derive(Serialize, Deserialize)]
pub struct CreateNewTodo{
    title:String,
    description: Option<String>,  
}

#[derive(Serialize,Deserialize,FromRow)]
pub struct  Todo {
    id: i32,
    title: String,
    description: Option<String>,
    status: String
}

#[derive(Serialize)]
pub struct TypeDbError{
    error: String
}

#[derive(Deserialize)]
pub struct Id {
    id : i32,
}

#[post("/todo/mark/completed")]
pub async fn mark_todo_completed(id: Json<Id>, db:Data<MySqlPool>) -> impl Responder{
    let res = sqlx::query("UPDATE todos
                                                        SET status =?
                                                        WHERE id =?")
    .bind("Completed")
    .bind(&id.id)
    .execute(&**db)
    .await;

    match res {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}


#[derive(Deserialize)]
pub struct UpdateDescriptionStruct{
    id : i32,
    description : String
}

#[post("/todo/description/update")]
pub async fn update_todo_description(body: Json<UpdateDescriptionStruct>, db:Data<MySqlPool>) -> impl Responder{
    let res = sqlx::query("UPDATE todos
                                                        SET description =?
                                                        WHERE id =?")
    .bind(&body.description)
    .bind(&body.id)
    .execute(&**db)
    .await;

    match res {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[derive(Deserialize)]
pub struct UpdateTitleStruct{
    id : i32,
    title : String
}

#[post("/todo/title/update")]
pub async fn update_todo_title(body: Json<UpdateTitleStruct>, db:Data<MySqlPool>) -> impl Responder{
    let res = sqlx::query("UPDATE todos
                                                        SET title =?
                                                        WHERE id =?")
    .bind(&body.title)
    .bind(&body.id)
    .execute(&**db)
    .await;

    match res {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[post("/todo/create")]
pub async fn create_new_todo(db:Data<MySqlPool>, body: Json<CreateNewTodo>) ->impl Responder{
    let response = sqlx::query
    (
        "INSERT INTO todos(title, description) values(?,?)"
    )
    .bind(&body.title)
    .bind(&body.description)
    .execute(&**db)
    .await;

    match response {
        Ok(id) => 
            HttpResponse::Created().json(Todo {
                id: id.last_insert_id() as i32,
                description: body.description.clone(),
                title: body.title.clone(),
                status: "New".to_string(),
            })
        ,
        Err(_e) => HttpResponse::InternalServerError().json(TypeDbError{
            error: _e.to_string(),
        }),
    }
}

#[get("/todos/all")]
pub async fn get_all_todos(db:Data<MySqlPool>) -> impl Responder {
    let res: Result<Vec<Todo>, Error> = sqlx::query_as("select 
    id, title, description, status from todos
    ")
    .fetch_all(&**db)
    .await;

    match res {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_e) => HttpResponse::InternalServerError().json(TypeDbError{
            error: _e.to_string(),

        })
    }
}