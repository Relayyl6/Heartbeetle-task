use actix_web::{web, HttpResponse, Responder};
use crate::models::{SharedState, CreateItem, Item, UpdateItem};
use crate::utils::write_to_file;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde_json::json;


impl From<CreateItem> for Item {
    fn from(c: CreateItem) -> Self {
        let now = Utc::now();
        Item {
            id: Uuid::new_v4(),
            name: c.name,
            price: c.price,
            quantity: c.quantity,
            description: c.description,
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }
}


// Items layer
// Create Item Handler
pub async fn create_item(
    state: web::Data<SharedState>,
    req: web::Json<CreateItem>
) -> impl Responder {
    let item: Item = req.into_inner().into();
    
    let mut s = state.lock().await;  //
    s.items.insert(item.id, item.clone());
    
    match write_to_file(&s).await {
        Ok(_) => HttpResponse::Created().json(item),
        Err(e) => {
            s.items.remove(&item.id);
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to persist item: {}", e)
            }))
        }
    }
}
// Get Item Handler
pub async fn get_item(
    state: web::Data<SharedState>,
    path: web::Path<Uuid>
) -> impl Responder {
    let item_id = path.into_inner();
    let s = state.lock().await;  // Lock acquired for reading
    match s.items.get(&item_id) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().json(json!({
            "error": format!("Item with id {} not found", item_id)
        }))
    }
}
// Update Item Handler
pub async fn update_item(
    state: web::Data<SharedState>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateItem>
) -> impl Responder {
    let item_id = path.into_inner();
    let dto = req.into_inner();
    let mut s = state.lock().await;  
    if let Some(item) = s.items.get_mut(&item_id) {
        // Apply updates
        if let Some(name) = dto.name {
            item.name = name;
        }
        if let Some(price) = dto.price {
            item.price = price;
        }
        if let Some(quantity) = dto.quantity {
            item.quantity = quantity;
        }
        if let Some(description) = dto.description {
            item.description = Some(description);
        }
        if let Some(active) = dto.is_active {
            item.is_active = active;
        }
        item.updated_at = Utc::now();
        let updated_item = item.clone();
        // Persist to file
        match write_to_file(&s).await {
            Ok(_) => HttpResponse::Ok().json(updated_item),
            Err(e) => HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to persist update: {}", e)
            }))
        }
    } else {
        HttpResponse::NotFound().json(json!({
            "error": format!("Item with id {} not found", item_id)
        }))
    }
}
// Delete Item Handler
pub async fn delete_item(
    state: web::Data<SharedState>,
    path: web::Path<Uuid>
) -> impl Responder {
    let item_id = path.into_inner();
    let mut s = state.lock().await;
    match s.items.remove(&item_id) {
        Some(deleted_item) => {
            // Persist to file
            match write_to_file(&s).await {
                Ok(_) => HttpResponse::Ok().json(json!({
                    "message": "Item deleted successfully",
                    "item": deleted_item
                })),
                Err(e) => {
                    // Rollback
                    s.items.insert(item_id, deleted_item);
                    HttpResponse::InternalServerError().json(json!({
                        "error": format!("Failed to persist deletion: {}", e)
                    }))
                }
            }
        },
        None => HttpResponse::NotFound().json(json!({
            "error": format!("Item with id {} not found", item_id)
        }))
    }
}
// List All Items Handler
pub async fn list_items(
    state: web::Data<SharedState>
) -> impl Responder {
    let s = state.lock().await;  // Lock acquired for reading
    let items: Vec<Item> = s.items.values().cloned().collect();
    HttpResponse::Ok().json(items)
    // Lock released here
}
