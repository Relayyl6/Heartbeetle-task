use sqlx::{PgPool, Error};
use uuid::Uuid;
use crate::models::{User, UserDB, CreateUser, UpdateUser, Order, OrderStatus, OrderDB, Item};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn create_user(
        &self,
        req: &CreateUser,
    ) -> Result<User, Error> {
        let user_db = sqlx::query_as!(
            UserDB,
            r#"
            INSERT INTO users (name, email)
            VALUES ($1, $2)
            RETURNING
                id,
                name,
                email,
                is_active,
                created_at,
                updated_at
            "#,
            req.name,
            req.email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(self.enrich_user(user_db).await?)
    }

    pub async fn get_user(
        &self,
        id: Uuid,
    ) -> Result<User, Error> {
        let user_db = sqlx::query_as!(
            UserDB,
            r#"
            SELECT
                id,
                name,
                email,
                is_active,
                created_at,
                updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(self.enrich_user(user_db).await?)
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        req: &UpdateUser,
    ) -> Result<User, Error> {
        let user_db = sqlx::query_as!(
            UserDB,
            r#"
            UPDATE users
            SET
                name = COALESCE($1, name),
                email = COALESCE($2, email),
                is_active = COALESCE($3, is_active)
            WHERE id = $4
            RETURNING
                id,
                name,
                email,
                is_active,
                created_at,
                updated_at
            "#,
            req.name.as_ref(),
            req.email.as_ref(),
            req.is_active,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(self.enrich_user(user_db).await?)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn list_users(&self) -> Result<Vec<User>, Error> {
        let users_db = sqlx::query_as!(
            UserDB,
            r#"
            SELECT
                id,
                name,
                email,
                is_active,
                created_at,
                updated_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut users = Vec::with_capacity(users_db.len());
        for user_db in users_db {
            users.push(self.enrich_user(user_db).await?);
        }

        Ok(users)
    }

    /// Populate orders separately (this is the key idea)
    async fn enrich_user(&self, user_db: UserDB) -> Result<User, Error> {
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
            user_db.id
        )
        .fetch_all(&self.pool)
        .await?;

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

        Ok(User {
            id: user_db.id,
            name: user_db.name,
            email: user_db.email,
            is_active: user_db.is_active,
            created_at: user_db.created_at,
            updated_at: user_db.updated_at,
            orders,
        })
    }

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
}
