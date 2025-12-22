## **Task 1 â€” Bubble Sort**

### **Goal:**
Implement bubble sort, generate pseudo-random data, and count swap operations.

### **Requirements:**
* The program must generate a random array size in the range **5â€“50**.
* Then generate that many random integers in the range **[-100, 100]**.
* Print the array **before sorting**.
* Implement **your own bubble sort** (no `.sort()`!).
* Count the number of **swap operations** (e.g., changing `[1, 5, 4]` â†’ `[1, 4, 5]` counts as **1 swap**).
* Print the sorted array and the total number of swaps.

### **Bonus points:**

* Count the number of comparisons.
* Early exit if the array is already sorted during a pass.

## **Task 2 â€” REST API with a JSON File as Storage**

### **Goal:**
Verify that the candidate can build a backend that uses routing, JSON serialization/deserialization, CRUD operations, and persistent storage in a local JSON file.

### **Description:**
Build a REST API in Rust (Native Rust, Actix Web, Axum, Rocket - any framework is allowed).
Instead of using an in-memory store, the API must **store all data in a single JSON file on disk**, e.g.:

```
data.json
```

The server must:

1. **Load** the JSON contents into memory when the application starts.
2. **Modify** the in-memory data as API requests come in.
3. **Write** the updated state back to the JSON file after every create/update/delete operation.

You may use:

```
Arc<Mutex<AppState>>
```

to keep the in-memory state synchronized across threads.

### **JSON Data Structure Example:**

```rs
struct AppState {
    users: Vec<User>
}

struct User {
    id: u32,
    name: String,
    orders: Vec<Order>
}

struct Order {
    order_id: u32,
    items: Vec<Item>
}

struct Item {
    id: u32,
    name: String,
    price: f64
}
```

### **Required Endpoints**

**GET**

* `/users` â€” return all users
* `/users/{id}` â€” return a single user
* `/users/{id}/orders/{order_id}` â€” return a specific order
* `/users/{id}/orders/{order_id}/items` â€” return item list

**POST**

* `/users` â€” create a new user
* `/users/{id}/orders` â€” create a new order
* `/users/{id}/orders/{order_id}/items` â€” add an item to an order

**PUT or PATCH**

* Update a user
* Update an order
* Update an item

**DELETE**

* Delete a user
* Delete an order
* Delete an item

---

### **Requirements:**

* **All data must be stored in a single JSON file** (e.g. `data.json`).
* On startup, the API **must load the file** into memory.
* Every write operation (POST, PUT, PATCH, DELETE) must **write the updated state back** to the file.
* IDs must be **auto-generated** (e.g., incremental counters or UUIDs).
* Proper error handling must be implemented:

  * 404 (user/order/item not found)
  * 400 (invalid data)
* No external database, no in-memory-only mode.

### **Bonus Points:**

* Logging middleware (log every request: method + path).
* Input validation (e.g., price > 0, user!=null etc.).
* Graceful handling of corrupted JSON (fallback to empty state).
* Unit tests for file load/save.

## **Task 3 â€” Repository Layer Using a Real MySQL Database (Rust)**

### **Goal:**
Verify that the candidate can use a real SQL database (MySQL), apply repository patterns, handle connections, perform CRUD operations, and maintain clean separation between logic and persistence.


### **Description:**
Create a Rust module named `repository` that defines traits for working with users (and optionally orders/items).

The application must use a **MySQL database** (local or containerized) and interact with it using any asynchronous Rust MySQL client (for example: `sqlx`, `tokio-mysql`, etc.). It's not neccessary for you to create an actual database, if you feel confident with your code; however we recommend setting up a simple database, just for the purpose of making sure that the program works properly.

The repository must abstract database operations behind traits so the rest of the application does not depend on SQL-specific code.

### **SQL Schema Required**

Create (at minimum) the following tables:

```
users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

orders (
    order_id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

items (
    item_id INT AUTO_INCREMENT PRIMARY KEY,
    order_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    price DOUBLE NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(order_id) ON DELETE CASCADE
);
```

You may extend the schema if needed.

### **Traits to Implement**

Example user trait:

```
#[async_trait]
trait UserRepository {
    async fn create_user(&self, name: String) -> Result<User, RepoError>;
    async fn get_user(&self, id: u32) -> Result<Option<User>, RepoError>;
    async fn update_user(&self, id: u32, name: String) -> Result<Option<User>, RepoError>;
    async fn delete_user(&self, id: u32) -> Result<bool, RepoError>;
}
```

You may add `OrderRepository` and `ItemRepository` traits as well.

---

### **Implementation Requirements**

1. **MySQL Repository**

You must implement these traits using ** SQL queries**:

* INSERT user, order, item
* SELECT with filters
* UPDATE user/order/item
* DELETE with foreign key constraints
* JOINs where appropriate

The repository must:

* Establish a MySQL connection pool
* Prepare queries using placeholders to prevent injection
* Return typed Rust structs
* Properly handle MySQL errors (duplicate keys, foreign key violations, etc.)

---

### **Final Required Behaviour**

* The REST API from **Task 2** must be updated to use **this MySQL repository** instead of RAM/JSON.
* The repository must be injected into the API using configuration or dependency injection.
* The API should not contain SQL-related logicâ€”only the repository should.

---

### **Bonus Points:**

* Implement multiple repositories (User, Order, Item).
* Add transaction support (e.g., delete user â†’ delete orders â†’ delete items).
* Use `sqlx` macros to check queries at compile time.
* Add unit tests or integration tests (e.g., using a test database).
* Add database migrations (e.g. using `sqlx migrate`).

## **Task 4 â€” Implement an Asynchronous Background Job Queue (Rust)**

### **Goal:**

Verify the candidateâ€™s ability to build **concurrent workers**, handle **async tasks**, manage **state**, and expose an API to work with a background queue.

This tests skills that real backend engineers need: synchronization, async runtime, task scheduling, queues, job states, etc.

---

### **Description:**

Extend the existing REST API from Task 2 & Task 3 by adding a **Job Queue System** that performs operations in the background.

You must implement:

### **1. A Job Queue Structure**

* Stored in memory (`Arc<Mutex<Vec<Job>>>`)
* Jobs must have:

  ```
  job_id: u64
  status: Pending | Running | Completed | Failed
  created_at: timestamp
  updated_at: timestamp
  payload: String (arbitrary JSON or text)
  result: Option<String>
  ```

### **2. A Worker System**

* A background Tokio task (spawned when API starts)
* Continuously checks the queue
* Executes jobs asynchronously
* Updates job status and result

### **3. REST API Endpoints**

**POST `/jobs`**

Adds a new job to the queue.
Example body:

```
{
  "payload": "generate_report_for_user:42"
}
```

Returns job_id.

**GET `/jobs/{id}`**

Returns the job status and result.

**GET `/jobs`**

Returns all jobs.

### **4. Required behavior**

* Only one job should be processed at a time (simple worker).
* The worker must simulate a long-running task (e.g. sleep 2â€“5 seconds).
* Jobs must transition through states properly.
* The job system must not block the main API thread.

### **Bonus points:**

* Add multiple workers processing in parallel.
* Limit the queue size.
* Add job retries on failure.
* Add job prioritization (low/medium/high).
* Add job expiration.
