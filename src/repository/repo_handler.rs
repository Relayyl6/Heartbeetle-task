use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::repository::items_db::ItemRepository;
use crate::repository::order_db::OrderRepository;
use crate::repository::users_db::UserRepository;
use crate::models::{CreateUser, UpdateUser, CreateItem, UpdateItem, CreateOrder, UpdateOrder, OrderStatus, StatusQuery};


// user db handler
pub async fn create_user(
    repo: web::Data<UserRepository>,
    req: web::Json<CreateUser>,
) -> impl Responder {
    match repo.create_user(&req).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            eprintln!("DB error creating user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create user"
            }))
        }
    }
}

pub async fn get_user(
    repo: web::Data<UserRepository>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id = path.into_inner();
    
    match repo.get_user(user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("User with id {} not found", user_id)
        })),
        Err(e) => {
            eprintln!("DB error fetching user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn update_user(
    repo: web::Data<UserRepository>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateUser>,
) -> impl Responder {
    let user_id = path.into_inner();
    
    match repo.update_user(user_id, &req).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("User with id {} not found", user_id)
        })),
        Err(e) => {
            eprintln!("DB error updating user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update user"
            }))
        }
    }
}

pub async fn delete_user(
    repo: web::Data<UserRepository>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id = path.into_inner();
    
    match repo.delete_user(user_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "User deleted successfully"
        })),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("User with id {} not found", user_id)
        })),
        Err(e) => {
            eprintln!("DB error deleting user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete user"
            }))
        }
    }
}

pub async fn list_users(
    repo: web::Data<UserRepository>,
) -> impl Responder {
    match repo.list_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("DB error listing users: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch users"
            }))
        }
    }
}








// item db handler
pub async fn create_item(
    repo: web::Data<ItemRepository>,
    req: web::Json<CreateItem>,
) -> impl Responder {
    match repo.create_item(&req).await {
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => {
            eprintln!("DB error creating item: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create item"
            }))
        }
    }
}

pub async fn get_item(
    repo: web::Data<ItemRepository>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let item_id = path.into_inner();
    
    match repo.get_item(item_id).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Item with id {} not found", item_id)
        })),
        Err(e) => {
            eprintln!("DB error fetching item: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn update_item(
    repo: web::Data<ItemRepository>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateItem>,
) -> impl Responder {
    let item_id = path.into_inner();
    
    match repo.update_item(item_id, &req).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Item with id {} not found", item_id)
        })),
        Err(e) => {
            eprintln!("DB error updating item: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update item"
            }))
        }
    }
}

pub async fn delete_item(
    repo: web::Data<ItemRepository>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let item_id = path.into_inner();
    
    match repo.delete_item(item_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Item deleted successfully"
        })),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Item with id {} not found", item_id)
        })),
        Err(e) => {
            eprintln!("DB error deleting item: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete item"
            }))
        }
    }
}

pub async fn list_items(
    repo: web::Data<ItemRepository>,
) -> impl Responder {
    match repo.list_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            eprintln!("DB error listing items: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch items"
            }))
        }
    }
}

pub async fn list_active_items(
    repo: web::Data<ItemRepository>,
) -> impl Responder {
    match repo.list_active_items().await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => {
            eprintln!("DB error listing active items: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch active items"
            }))
        }
    }
}


// order db handler
pub async fn create_order(
    repo: web::Data<OrderRepository>,
    req: web::Json<CreateOrder>,
) -> impl Responder {
    match repo.create_order(&req).await {
        Ok(order) => HttpResponse::Created().json(order),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User or one or more items not found"
        })),
        Err(e) => {
            eprintln!("DB error creating order: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create order"
            }))
        }
    }
}

pub async fn get_order(
    repo: web::Data<OrderRepository>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let order_id = path.into_inner();
    
    match repo.get_order(order_id).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Order with id {} not found", order_id)
        })),
        Err(e) => {
            eprintln!("DB error fetching order: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn get_order_with_items(
    repo: web::Data<OrderRepository>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let order_id = path.into_inner();
    
    match repo.get_order_with_items(order_id).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Order with id {} not found", order_id)
        })),
        Err(e) => {
            eprintln!("DB error fetching order with items: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn update_order(
    repo: web::Data<OrderRepository>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateOrder>,
) -> impl Responder {
    let order_id = path.into_inner();
    
    match repo.update_order(order_id, &req).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Order or one or more items not found"
        })),
        Err(e) => {
            eprintln!("DB error updating order: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update order"
            }))
        }
    }
}

pub async fn delete_order(
    repo: web::Data<OrderRepository>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let order_id = path.into_inner();
    
    match repo.delete_order(order_id).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Order deleted successfully"
        })),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Order with id {} not found", order_id)
        })),
        Err(e) => {
            eprintln!("DB error deleting order: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete order"
            }))
        }
    }
}

pub async fn list_orders(
    repo: web::Data<OrderRepository>,
) -> impl Responder {
    match repo.list_orders().await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => {
            eprintln!("DB error listing orders: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch orders"
            }))
        }
    }
}

pub async fn get_orders_by_user(
    repo: web::Data<OrderRepository>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let user_id = path.into_inner();
    
    match repo.get_orders_by_user(user_id).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => {
            eprintln!("DB error fetching orders by user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch orders"
            }))
        }
    }
}

pub async fn get_orders_by_status(
    repo: web::Data<OrderRepository>,
    query: web::Query<StatusQuery>,
) -> impl Responder {
    match repo.get_orders_by_status(query.status.clone()).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => {
            eprintln!("DB error fetching orders by status: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch orders"
            }))
        }
    }
}

