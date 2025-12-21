use actix_web::{web, HttpResponse, Responder};
use crate::models::{SharedState, CreateUser, User, Order, UpdateUser};
use crate::utils::{write_to_file};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// Data Transfer Object layer
impl From<CreateUser> for User{
    fn from(c: CreateUser) -> Self {
        let now = Utc::now();
        User {
            id: Uuid::new_v4(),
            name: c.name,
            email: c.email,
            orders: vec![],
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }
}

// User layer
pub async fn create_user(
    state: web::Data<SharedState>,
    req: web::Json<CreateUser>,
) -> impl Responder {
//  let dto: CreateUser = req.into_inner();  // Deserializes JSON to CreateUser
//  let user: User = dto.into();             // Calls From<CreateUser> for User
    let user: User = req.into_inner().into();

    // update in-memory state
    let mut s = state.lock().await;

    // Store user
    s.users.insert(user.id, user.clone());
    match write_to_file(&s).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "message": "user successfully signed up",
            "user": user,
        })),
        Err(err) => {
            s.users.remove(&user.id);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to pesist user: {}", err)
            }))
        }
    }
}

pub async fn get_user(
    state: web::Data<SharedState>,
    path: web::Path<Uuid>
) -> impl Responder {
    let user_id = path.into_inner();
    let s = state.lock().await;
    match s.users.get(&user_id) {
        Some(user) => {
            let orders: Vec<Order> = user.orders.iter()
                .filter_map(|order| s.orders.get(&order.id).cloned())
                .collect();

            let user_details = User {
                id: user.id,
                name: user.name.clone(),
                email: user.email.clone(),
                orders,
                created_at: user.created_at,
                updated_at: user.updated_at,
                is_active: user.is_active,
            };
            HttpResponse::Ok().json(user_details)
        },
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("User with id {} not found", user_id)
        }))
    }
}

pub async fn update_user(
    state: web::Data<SharedState>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateUser>
) -> impl Responder {
    let user_id = path.into_inner();
    let dto = req.into_inner();
    let mut s = state.lock().await;
    if let Some(user) = s.users.get_mut(&user_id) {
        // Apply updates this format is mre readable
        if let Some(name) = dto.name { // alternate syntax: dto.name.map(|n| item.name = n);
            user.name = name;
        }
        if let Some(email) = dto.email {
            user.email = email;
        }
        if let Some(active) = dto.is_active {
            user.is_active = active;
        }
        user.updated_at = Utc::now();
        let updated_user = user.clone();
        // Persist to file
        match write_to_file(&s).await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                "message": "user successfully updated",
                "updated user": updated_user,
            })),
            Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to persist update: {}", e)
            }))
        }
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("User with id {} not found", user_id)
        }))
    }
}
pub async fn delete_user(
    state: web::Data<SharedState>,
    path: web::Path<Uuid>
) -> impl Responder {
    let user_id = path.into_inner();
    let mut s = state.lock().await;
    match s.users.remove(&user_id) {
        Some(deleted_user) => {
            // Persist to file
            match write_to_file(&s).await {
                Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                    "message": "User deleted successfully",
                    "user": deleted_user
                })),
                Err(e) => {
                    // Rollback - re-insert the user
                    s.users.insert(user_id, deleted_user);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": format!("Failed to persist deletion: {}", e)
                    }))
                }
            }
        },
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("User with id {} not found", user_id)
        }))
    }
}
pub async fn list_users(
    state: web::Data<SharedState>
) -> impl Responder {
    let s = state.lock().await;
    let users: Vec<User> = s.users.values().cloned().collect();
    HttpResponse::Ok().json(users)
}
