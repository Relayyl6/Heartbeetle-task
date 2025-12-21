use sqlx::{Pool, PgPool, Error};
use uuid::Uuid;
use crate::models::{Item, CreateItem, UpdateItem};

pub struct ItemRepository {
    pool: PgPool
}

impl ItemRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            pool: pool.clone()
        }
    }

    pub async fn create_item(
        &self,
        req: &CreateItem
    ) -> Result<Item, Error> {
        let item = sqlx::query_as!(
            Item,
            r#"
            INSERT INTO items (name, price, quantity, description)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            req.name,
            req.price,
            req.quantity,
            req.description
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(item)
    }

    pub async fn get_item(
        &self,
        id: Uuid,
    ) -> Result<Item, Error> {
        let item = sqlx::query_as!(
            Item,
            r#"
            SELECT * FROM items
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(item)
    }

    pub async fn update_item(
        &self,
        id: Uuid,
        req: &UpdateItem
    ) -> Result<Item, Error> {
        let item = sqlx::query_as!(
            Item,
            r#"
            UPDATE items
            SET
                name = COALESCE($1, name),
                price = COALESCE($2, price),
                quantity = COALESCE($3, quantity),
                description = COALESCE($4, description),
                is_active = COALESCE($5, is_active),
                updated_at = now()
            WHERE id = $6
            RETURNING *
            "#,
            req.name.as_ref(),
            req.price,
            req.quantity,
            req.description.as_ref(),
            req.is_active,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(item)
    }

    pub async fn delete_item(
        &self,
        id: Uuid,
    ) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM items WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_items(&self) -> Result<Vec<Item>, Error> {
        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT * FROM items
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    // Optional: Get only active items
    pub async fn list_active_items(&self) -> Result<Vec<Item>, Error> {
        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT * FROM items
            WHERE is_active = true
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }
}