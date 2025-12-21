use uuid::Uuid;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use tokio::sync::Mutex;
use std::sync::Arc;

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


#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum JobPriority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub job_id: u64,
    pub status: JobStatus,
    pub payload: String,
    pub result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<JobPriority>,  // Bonus feature
    pub retries: u32,  // Bonus feature
    pub max_retries: u32,  // Bonus feature
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,  // Bonus feature
}

#[derive(Debug, Deserialize)]
pub struct CreateJob {
    pub payload: String,
    #[serde(default)]
    pub priority: Option<JobPriority>,
    #[serde(default)]
    pub max_retries: Option<u32>,
    #[serde(default)]
    pub ttl_seconds: Option<i64>,  // Time to live in seconds
}

#[derive(Debug, Clone)]
pub struct JobQueue {
    pub jobs: Arc<Mutex<Vec<Job>>>,
    pub next_id: Arc<Mutex<u64>>,
    pub max_queue_size: usize,  // Bonus feature
}

#[derive(Debug, Deserialize)]
pub struct StatusJobQuery {
    pub status: JobStatus,
}
