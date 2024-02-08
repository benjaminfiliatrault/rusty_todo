use std::env;

use serde::{Deserialize, Serialize};
use axum::{routing::get, Router, extract::{State, RawBody}, body, Json};
use sqlx::{Pool, migrate::MigrateDatabase, Result};
use sqlx::postgres::Postgres;

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    

    let state = AppState {
        pool, 
    };

    // build our application with a single route
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/:id",
            get(get_todo_by_id).put(update_todo).delete(delete_todo),
        )
        .with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_db() -> Result<Pool<Postgres>> {
    use std::path::Path;

    // Create the database
    let db_url = "postgres://host.docker.internal:5432";

    if !sqlx::postgres::Postgres::database_exists(&db_url).await? {
        sqlx::postgres::Postgres::create_database(&db_url).await?;
    }

    // Connect to the database 
    let db = Pool::<Postgres>::connect(db_url).await.unwrap();

    // Migrate the database
    let migrations = if env::var("RUST_ENV") == Ok("production".to_string()) {
        // Productions migrations dir
        std::env::current_exe()?.join("./migrations")
    } else {
        // Development migrations dir
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&crate_dir)
            .join("./migrations")
    };

    sqlx::migrate::Migrator::new(migrations)
        .await?
        .run(&db)
        .await?;

    Ok(db)
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

async fn list_todos() -> &'static str {
    "List TODOS"
}

async fn get_todo_by_id() -> &'static str {
    "Get TODO"
}


#[derive(Serialize, Deserialize, Debug)]
struct CreateTodo {
    title: String,
}
async fn create_todo(
    Json(payload): Json<CreateTodo>,
    State(state): State<AppState>,
) -> &'static str { 

    sqlx::query("INSERT INTO todos");

    
 "Create Todo"
}

async fn delete_todo() -> &'static str {
    "Delete TODOS"
}

async fn update_todo() -> &'static str {
    "Update TODO"
}
