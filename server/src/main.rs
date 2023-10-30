use axum::{http::StatusCode, Json, Router, routing::get};
use rusqlite::{Connection, Result, };
use serde::Serialize;

#[derive(Serialize)]
struct Todo {
    id: i32,
    text: String,
}

#[derive(Serialize)]
struct TodoResponse {
    todos: Vec<Todo>,
}

async fn get_todos() -> Result<Json<TodoResponse>, StatusCode> {
    let conn = Connection::open("todos.db").unwrap();

    let mut stmt = conn.prepare("SELECT id, text FROM todos").unwrap();
    let todos = stmt.query_map(
        []
        , |row| {
            Ok(Todo {
                id: row.get(0).unwrap(),
                text: row.get(1).unwrap(),
            })
        }).unwrap();

    let todos: Vec<Todo> = todos.map(|todo| todo.unwrap()).collect();

    //let resp = serde_json::to_string(&todos).unwrap();
    let resp = TodoResponse { todos };

    Ok(Json(resp))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/todos", get(get_todos));
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
