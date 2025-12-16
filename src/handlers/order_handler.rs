use actix_web::{web, HttpResponse, Responder};
use crate::models::{SharedState, CreateOrder, Order};
use crate::utils::write_to_file;
use chrono::{DateTime, Utc};
use uuid::Uuid;

impl From<CreateOrder> for Order {
    fn from(c: CreateOrder) -> Self {
        let now = Utc::now();
        Order {
            id: Uuid::new_v4(),
            user_id: c.user_id,
            items: c.item_ids,
            amount: 0.0,
            status: OrderStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
}

// Order layer 
pub async fn create_order(
    state: web::Data<SharedState>,
    req: web::Json<CreateOrder>
) -> impl Responder {
    let dto = req.into_inner();
    let user_id = dto.user_id;
    let mut s = state.lock().await;
    // Verify user exists
    if !s.users.contains_key(&user_id) {
        return HttpResponse::NotFound().json(json!({
            "error": format!("User with id {} not found", user_id)
        }));
    }
    // Calculate total amount from items
    let mut total_amount = 0.0;
    for item_id in &dto.item_ids {
        match s.items.get(item_id) {
            Some(item) => {
                if !item.is_active {
                    return HttpResponse::BadRequest().json(json!({ // 400 BadRequest
                        "error": format!("Item {} is not available", item_id)
                    }));
                }
                total_amount += item.price;
            },
            None => {
                return HttpResponse::NotFound().json(json!({
                    "error": format!("Item with id {} not found", item_id)
                }));
            }
        }
    }
    // Create order
    let mut order: Order = dto.into();
    order.amount = total_amount;  // Set calculated amount
    // Store order
    s.orders.insert(order.id, order.clone());
    // Persist to file
    match write_to_file(&s).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "message": "Order sucessfully created",
            "order": order 
        })),
        Err(e) => {
            s.orders.remove(&order.id);
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to persist order: {}", e)
            }))
        }
    }
}

pub async fn update_order(
    state: web::Data<SharedState>,
    path: web::Path<Uuid>,
    req: web::Json<UpdateOrder>
) -> impl Responder {
    let order_id = path.into_inner();
    let dto = req.into_inner();
    let mut s = state.lock().await;
    if let Some(order) = s.orders.get_mut(&order_id) {
        // Update item_ids if provided
        if let Some(new_item_ids) = dto.item_ids { // add as many items to the list as pssible // more flexible thn adding an id in the path
            // Recalculate amount with new items
            let mut new_amount = 0.0;
            for item_id in &new_item_ids {
                match s.items.get(item_id) {
                    Some(item) => {
                        if !item.is_active {
                            return HttpResponse::BadRequest().json(json!({
                                "error": format!("Item {} is not available", item_id)
                            }));
                        }
                        new_amount += item.price; // calculate the new amount of the order
                    },
                    None => {
                        return HttpResponse::NotFound().json(json!({
                            "error": format!("Item with id {} not found", item_id)
                        }));
                    }
                }
            }
            order.items = new_item_ids;
            order.amount = new_amount;
        }
        // Update status if provided
        if let Some(new_status) = dto.status {
            order.status = new_status;
        }
        order.updated_at = Utc::now();
        let updated_order = order.clone();
        // Persist to file
        match write_to_file(&s).await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                "message": "Order sucessfully created",
                "updated order": updated_order 
            })),
            Err(e) => HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to persist update: {}", e)
            }))
        }
    } else {
        HttpResponse::NotFound().json(json!({
            "error": format!("Order with id {} not found", order_id)
        }))
    }
}

pub async fn get_order_with_details(
    state: web::Data<SharedState>,
    path: web::Path<Uuid>
) -> impl Responder {
    let order_id = path.into_inner();
    let s = state.lock().await;
    match s.orders.get(&order_id) {
        Some(order) => {
            // Fetch full item details
            let items: Vec<Item> = order.items.iter()
                .filter_map(|item_id| s.items.get(item_id).cloned())
                .collect();
            let order_details = Order {
                id: order.id,
                user_id: order.user_id,
                items,
                amount: order.amount,
                status: order.status.clone(), // .clone() minizes peformance but I cut corners since performace cost is primary to gettign the full items within an order
                created_at: order.created_at,
                updated_at: order.updated_at,
            };
            HttpResponse::Ok().json(order_details)
        },
        None => HttpResponse::NotFound().json(json!({
            "error": format!("Order with id {} not found", order_id)
        }))
    }
}

pub async fn list_orders(
    state: web::Data<SharedState>
) -> impl Responder {
    let s = state.lock().await;
    let orders: Vec<Order> = s.orders.values().cloned().collect();
    HttpResponse::Ok().json(orders)
}