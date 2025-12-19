use uuid::Uuid;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<Item>,
    pub amount: f64,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    Pending,
    Paid,
    Cancelled,
    Shipping,
    Delivered,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub orders: Vec<Order>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}


#[derive(Debug)]
pub struct AppState {
    pub users: HashMap<Uuid, User>,
    pub orders: HashMap<Uuid, Order>,
    pub items: HashMap<Uuid, Item>,
}

pub type SharedState = Arc<Mutex<AppState>>;




// to create and update an item

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub price: Option<f64>,
    pub quantity: Option<u32>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

// to create an update an order
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrder {
    pub user_id: Uuid,
    pub item_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrder {
    pub item_ids: Option<Vec<Uuid>>,
    pub status: Option<OrderStatus>,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
}

// Helper struct for status query
#[derive(Debug, Deserialize)]
pub struct StatusQuery {
    pub status: OrderStatus,
}