use sqlx::{Pool, PgPool, Error};
use uuid::Uuid;
use crate::models::{Item, CreateItem, UpdateItem, Order, CreateOrder, UpdateOrder, OrderStatus};


pub struct UserRepository {
    pool: PgPool
}

impl UserRepository {
    pub fn new(
        pool: &PgPool
    ) -> Self {
        Self {
            pool: pool.clone()
        }
    }

    pub async fn create_user(
        &self,
        req: &CreateUser
    ) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
                r#"
                INSERT INTO users (name, email)
                VALUES ($1, $2)
                RETURNING *
                "#,
            req.name,
            req.email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user(
        &self,
        id: Uuid,
    ) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
                r#"
                SELECT * FROM users
                WHERE id = $1
                "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        req: &UpdateUser
    ) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
                r#"
                UPDATE users
                SET
                    name = COALESCE($1, name),
                    email = COALESCE($2, email),
                    is_active = COALESCE($3, is_active)
                WHERE id = $4
                RETURNING *
                "#,
            req.name.as_ref(),
            req.email.as_ref(),
            req.is_active,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn delete_user(
        &self,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM users WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }
}