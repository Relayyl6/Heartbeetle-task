mod handlers;
mod utils;
mod models;

use crate::handlers::item_handler;
use crate::handlers::user_handler;
use crate::handlers::order_handler;

use std::fs::File;
use actix_web::{web, App, HttpRequest, HttpServer};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::utils::{load_data_from_file, generate_random_array, bubble_sort};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // task 1 layer
    let mut data = generate_random_array();
    // println!("Before sorting: {:?}", data);
    // pass as a slice
    bubble_sort(&mut data);
    println!("After sorting: {:?}", data);

    // task 2 layer

    // Load data from file
    let initial_state = load_data_from_file().await
        .expect("Failed to load initial state");
    
    // Wrap in Arc<Mutex<>> for shared state
    let shared_state = web::Data::new(Arc::new(Mutex::new(initial_state)));
    
    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone()) // Share state across all workers // .clone() has a time complextity of O(1) here but under the hook is still preformace effective when wrapped around web::Data
            // User routes
            .route("/users", web::post().to(user_handler::create_user))
            .route("/users", web::get().to(user_handler::list_users))
            .route("/users/{id}", web::get().to(user_handler::get_user))
            .route("/users/{id}", web::put().to(user_handler::update_user))
            .route("/users/{id}", web::delete().to(user_handler::delete_user))
            // Item routes
            .route("/items", web::post().to(item_handler::create_item))
            .route("/items", web::get().to(item_handler::list_items))
            .route("/items/{id}", web::get().to(item_handler::get_item))
            .route("/items/{id}", web::put().to(item_handler::update_item))
            .route("/items/{id}", web::delete().to(item_handler::delete_item))
            // Order routes
            .route("/orders", web::post().to(order_handler::create_order))
            .route("/orders", web::get().to(order_handler::list_orders))
            .route("/orders/{id}", web::get().to(order_handler::get_order))
            .route("/orders/{id}", web::put().to(order_handler::update_order))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}