use sqlx::{PgPool, Error};
use uuid::Uuid;
use crate::models::{Order, OrderDB, CreateOrder, UpdateOrder, OrderStatus, Item};


pub struct OrderRepository {
    pool: PgPool
}

impl OrderRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            pool: pool.clone()
        }
    }

    pub async fn create_order(
        &self,
        req: &CreateOrder
    ) -> Result<Order, Error> {
        // Start a transaction
        let mut tx = self.pool.begin().await?;

        // 1. Create the order
        let order = sqlx::query_as!(
            OrderDB,
            r#"
            INSERT INTO orders (user_id, amount)
            VALUES ($1, $2)
            RETURNING
                id,
                user_id,
                amount,
                status as "status: OrderStatus", -- default calculated in the db as pending
                created_at,
                updated_at
            "#,
            req.user_id,
            0.0, // Amount will be calculated
        )
        .fetch_one(&mut *tx)
        .await?;

        // 2. Insert order items and calculate total
        let mut total_amount = 0.0;
        for item_id in &req.item_ids {
            // Get item price
            let item = sqlx::query!(
                r#"
                SELECT price FROM items
                WHERE id = $1 AND is_active = true
                "#,
                item_id
            )
            .fetch_one(&mut *tx)
            .await?;

            total_amount += item.price;

            // Insert into order_items junction table
            sqlx::query!(
                r#"
                INSERT INTO order_items (order_id, item_id)
                VALUES ($1, $2)
                "#,
                order.id,
                item_id
            )
            .execute(&mut *tx)
            .await?;
        }

        // 3. Update order with calculated amount
        let final_order = sqlx::query_as!(
            OrderDB,
            r#"
            UPDATE orders
            SET amount = $1
            WHERE id = $2
            RETURNING
                id,
                user_id,
                amount,
                status as "status: OrderStatus",
                created_at,
                updated_at
            "#,
            total_amount,
            order.id
        )
        .fetch_one(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        let items = self.get_order_items(final_order.id).await?;
    
        Ok(
            Order {
                id: final_order.id,
                user_id: final_order.user_id,
                items,
                amount: final_order.amount,
                status: final_order.status,
                created_at: final_order.created_at,
                updated_at: final_order.updated_at,
            }
        )
    }

    pub async fn get_order(
        &self,
        id: Uuid,
    ) -> Result<Order, Error> {
        let order = sqlx::query_as!(
            OrderDB,
            r#"
            SELECT 
                id,
                user_id,
                amount,
                status as "status: OrderStatus",
                created_at,
                updated_at
            FROM orders
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        let items = self.get_order_items(order.id).await?;
    
        Ok(
            Order {
                id: order.id,
                user_id: order.user_id,
                items,
                amount: order.amount,
                status: order.status,
                created_at: order.created_at,
                updated_at: order.updated_at,
            }
        )
    }

    // Get order with item details
    pub async fn get_order_with_items(
        &self,
        id: Uuid,
    ) -> Result<Order, Error> {
        // Get the order
        let order = self.get_order(id).await?;

        // Get associated items
        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT i.*
            FROM items i
            INNER JOIN order_items oi ON i.id = oi.item_id
            WHERE oi.order_id = $1
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(Order {
            id: order.id,
            user_id: order.user_id,
            items,
            amount: order.amount,
            status: order.status,
            created_at: order.created_at,
            updated_at: order.updated_at,
        })
    }

    pub async fn update_order(
        &self,
        id: Uuid,
        req: &UpdateOrder
    ) -> Result<Order, Error> {
        let mut tx = self.pool.begin().await?;

        // Update items if provided
        if let Some(ref item_ids) = req.item_ids {
            // Delete existing order_items
            sqlx::query!(
                "DELETE FROM order_items WHERE order_id = $1",
                id
            )
            .execute(&mut *tx)
            .await?;

            // Calculate new amount and insert new items
            let mut new_amount = 0.0;
            for item_id in item_ids {
                let item = sqlx::query!(
                    r#"
                    SELECT price FROM items
                    WHERE id = $1 AND is_active = true
                    "#,
                    item_id
                )
                .fetch_one(&mut *tx)
                .await?;

                new_amount += item.price;

                sqlx::query!(
                    r#"
                    INSERT INTO order_items (order_id, item_id)
                    VALUES ($1, $2)
                    "#,
                    id,
                    item_id
                )
                .execute(&mut *tx)
                .await?;
            }

            // Update amount
            sqlx::query!(
                "UPDATE orders SET amount = $1, updated_at = now() WHERE id = $2",
                new_amount,
                id
            )
            .execute(&mut *tx)
            .await?;
        }

        // Update the order
        let order = sqlx::query_as!(
            OrderDB,
            r#"
            UPDATE orders
            SET
                status = COALESCE($1, status),
                updated_at = now()
            WHERE id = $2
            RETURNING
                id,
                user_id,
                amount,
                status as "status: OrderStatus",
                created_at,
                updated_at
            "#,
            req.status.as_ref() as Option<&OrderStatus>,
            id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        // Convert OrderDB to Order by fetching items for each
        let items = self.get_order_items(order.id).await?;
    
        Ok(
            Order {
                id: order.id,
                user_id: order.user_id,
                items,
                amount: order.amount,
                status: order.status,
                created_at: order.created_at,
                updated_at: order.updated_at,
            }
        )
    }

    pub async fn delete_order(
        &self,
        id: Uuid,
    ) -> Result<(), Error> {
        // order_items will be deleted automatically due to CASCADE
        sqlx::query!(
            "DELETE FROM orders WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_orders(&self) -> Result<Vec<Order>, Error> {
        let orders_db = sqlx::query_as!(
            OrderDB,
            r#"
            SELECT 
                id,
                user_id,
                amount,
                status as "status: OrderStatus",
                created_at,
                updated_at
            FROM orders
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // Convert OrderDB to Order by fetching items for each
        let mut orders = Vec::new();
        for order_db in orders_db {
            let items = self.get_order_items(order_db.id).await?;
            orders.push(Order {
                id: order_db.id,
                user_id: order_db.user_id,
                items,
                amount: order_db.amount,
                status: order_db.status,
                created_at: order_db.created_at,
                updated_at: order_db.updated_at,
            });
        }
    
        Ok(orders)
    }

    // Get orders by user
    pub async fn get_orders_by_user(
        &self,
        user_id: Uuid
    ) -> Result<Vec<Order>, Error> {
        let orders_db = sqlx::query_as!(
            OrderDB,
            r#"
            SELECT
                id,
                user_id,
                amount,
                status as "status: OrderStatus",
                created_at,
                updated_at
            FROM orders
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
    
        // Convert OrderDB to Order by fetching items for each
        let mut orders = Vec::new();
        for order_db in orders_db {
            let items = self.get_order_items(order_db.id).await?;
            orders.push(Order {
                id: order_db.id,
                user_id: order_db.user_id,
                items,
                amount: order_db.amount,
                status: order_db.status,
                created_at: order_db.created_at,
                updated_at: order_db.updated_at,
            });
        }
    
        Ok(orders)
    }
    
    // Helper method to get items for an order
    async fn get_order_items(&self, order_id: Uuid) -> Result<Vec<Item>, Error> {
        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT i.*
            FROM items i
            INNER JOIN order_items oi ON i.id = oi.item_id
            WHERE oi.order_id = $1
            "#,
            order_id
        )
        .fetch_all(&self.pool)
        .await?;
    
        Ok(items)
    }

    // Get orders by status
    pub async fn get_orders_by_status(
        &self,
        status: OrderStatus
    ) -> Result<Vec<Order>, Error> {
        let orders_db = sqlx::query_as!(
            OrderDB,
            r#"
            SELECT 
                id,
                user_id,
                amount,
                status as "status: OrderStatus",
                created_at,
                updated_at
            FROM orders
            WHERE status = $1
            ORDER BY created_at DESC
            "#,
            status as OrderStatus
        )
        .fetch_all(&self.pool)
        .await?;

        // Convert OrderDB to Order by fetching items for each
        let mut orders = Vec::new();
        for order_db in orders_db {
            let items = self.get_order_items(order_db.id).await?;
            orders.push(Order {
                id: order_db.id,
                user_id: order_db.user_id,
                items,
                amount: order_db.amount,
                status: order_db.status,
                created_at: order_db.created_at,
                updated_at: order_db.updated_at,
            });
        }

        Ok(orders)
    }
}

