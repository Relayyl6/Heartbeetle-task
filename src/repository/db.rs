use sqlx::{Pool, Postgres, PgPool, Error};
use std::env;
use uuid::Uuid;
use dotenvy::dotenv;
use crate::models::{CreateUser, UpdateUser, User};

pub async fn get_db_pool() -> Pool<Postgres> {
    // dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let database_url = "".to_string();  // explicitly set my database url for the sake of demonstrating th etask, normally it would be in the .env
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres")
}

pub struct Repository {
    pool: PgPool
}

impl Repository {
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
        .fetch_one(pool)
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
                "#,
            req.name.as_ref(),
            req.email.as_ref(),
            req.is_active
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