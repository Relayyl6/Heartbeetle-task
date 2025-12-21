mod handlers;
mod utils;
mod models;
mod repository;
mod jobs;

use crate::handlers::item_handler;
use crate::handlers::user_handler;
use crate::handlers::order_handler;
// use crate::db::get_db_pool;
use crate::repository::db;
use crate::repository::items_db::ItemRepository;
use crate::repository::order_db::OrderRepository;
use crate::repository::users_db::UserRepository;
// handler function for the task 3
use crate::repository::repo_handler;

use crate::jobs::workers::spawn_workers;
use crate::jobs::jobs_done;
use crate::jobs::handler;

use crate::models::JobQueue;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use std::env;
use dotenvy;
use tokio::sync::Mutex;
use crate::utils::{load_data_from_file, generate_random_array, bubble_sort};
use sqlx::PgPool;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
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
    
    // task 3 wrapper
    let pool = db::get_db_pool().await;
    sqlx::migrate!("./migrations").run(&pool).await.expect("Migrations Failed");
    // let repo = web::Data::new(db::new(pool.clone()));
    let user_repo = web::Data::new(UserRepository::new(&pool));
    let items_repo = web::Data::new(ItemRepository::new(&pool));
    let order_repo = web::Data::new(OrderRepository::new(&pool));

    let port = env::var("SERVICE_PORT").unwrap_or_else(|_| "3003".into()); // default 3003

    // task 4 layer
    // Initialize job queue
    let max_queue_size = 100;  // Bonus: Queue size limit
    let job_queue: Arc<JobQueue> = Arc::new(JobQueue::new(max_queue_size));

    // Spawn workers
    let num_workers = 3;  // Change to 1 for single worker
    spawn_workers(num_workers, job_queue.clone());

    println!("Started {} worker(s)", num_workers);
    println!("Max queue size: {}", max_queue_size);

    // Start HTTP server
    let queue_data: web::Data<JobQueue> = web::Data::from(job_queue);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone()) // Share state across all workers // .clone() has a time complextity of O(1) here but under the hook is still preformace effective when wrapped around web::Data
            .app_data(user_repo.clone())
            .app_data(items_repo.clone())
            .app_data(order_repo.clone())
            .app_data(queue_data.clone())

            // task 2 layer - CRUD to the JSON layer
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
            .route("/orders/{id}", web::get().to(order_handler::get_order_with_details))
            .route("/orders/{id}", web::put().to(order_handler::update_order))

            // task 3 layer - CRUD to the db layer
            // User routes
            .route("/db/users", web::post().to(repo_handler::create_user))
            .route("/db/users", web::get().to(repo_handler::list_users))
            .route("/db/users/{id}", web::get().to(repo_handler::get_user))
            .route("/db/users/{id}", web::put().to(repo_handler::update_user))
            .route("/db/users/{id}", web::delete().to(repo_handler::delete_user))
            // Item routes
            .route("/db/items", web::post().to(repo_handler::create_item))
            .route("/db/items", web::get().to(repo_handler::list_items))
            .route("/db/items/active", web::get().to(repo_handler::list_active_items))
            .route("/db/items/{id}", web::get().to(repo_handler::get_item))
            .route("/db/items/{id}", web::put().to(repo_handler::update_item))
            .route("/db/items/{id}", web::delete().to(repo_handler::delete_item))
            // Order routes
            .route("/db/orders", web::post().to(repo_handler::create_order))
            .route("/db/orders", web::get().to(repo_handler::list_orders))
            .route("/db/orders/{id}", web::get().to(repo_handler::get_order))
            .route("/db/orders/{id}/details", web::get().to(repo_handler::get_order_with_items))
            .route("/db/orders/{id}", web::put().to(repo_handler::update_order))
            .route("/db/orders/{id}", web::delete().to(repo_handler::delete_order))
            .route("/db/orders/user/{user_id}", web::get().to(repo_handler::get_orders_by_user))
            .route("/db/orders/status", web::get().to(repo_handler::get_orders_by_status)) // ?status=Pending

            // task 4 routes
            .route("/jobs", web::post().to(handler::create_job))
            .route("/jobs", web::get().to(handler::list_jobs))
            .route("/jobs/{id}", web::get().to(handler::get_job))
            .route("/jobs/status", web::get().to(handler::list_jobs_by_status))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}