use std::sync::Arc;

use futures::future::try_join_all;
use nanoid::generate;
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use crate::crypto::Crypto;
use crate::errors::TalliiError;

use super::models::{InviteCode, NewUser, PublicUser, User};

pub struct InviteCodeRepository {
    pool: Arc<PgPool>,
}

impl InviteCodeRepository {
    /// Fetches a database pool connection to use for querying
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Checks if the provided invite code is valid
    pub async fn is_valid(&self, id: &String) -> Result<bool, TalliiError> {
        // get the invite code, if it exists
        let invite_code =
            sqlx::query_as::<_, InviteCode>("select * from invite_codes where id = $1")
                .bind(id)
                .fetch_optional(&*self.pool)
                .await?;

        // if invite code doesnt exist return false
        if invite_code.is_none() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Gets all invite codes
    /// TODO: Hide this behind the citadel
    pub async fn get_all(&self) -> Result<Vec<InviteCode>, TalliiError> {
        let all_invite_codes = sqlx::query_as::<_, InviteCode>("select * from invite_codes")
            .fetch_all(&*self.pool)
            .await?;

        Ok(all_invite_codes)
    }

    /// Creates invite codes for the specified amount
    /// Note, this query may not be the most performant query
    /// because it is not a bulk insert but that is fine.
    /// TODO: Hide this behind the citadel
    pub async fn create_many(&self, amount: i32) -> Result<(), TalliiError> {
        let mut new_codes_queries = Vec::new();

        // generate invite codes for the given amount
        for _ in 0..amount {
            let code = generate(6);

            new_codes_queries.push(
                sqlx::query("insert into invite_codes (id) values ($1)")
                    .bind(code)
                    .execute(&*self.pool),
            )
        }

        try_join_all(new_codes_queries).await?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    /// Fetches a database pool connection to use for querying
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Fetches a user with the provided email
    pub async fn get_by_email(&self, email: &String) -> Result<Option<User>, TalliiError> {
        let user_with_email = sqlx::query_as::<_, User>("select * from users where email = $1")
            .bind(email)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(user_with_email)
    }

    /// Fetches a username that matches the provided username and user_id.
    pub async fn get_by_username_and_id(
        &self,
        user_id: &i32,
        username: &String,
    ) -> Result<Option<User>, TalliiError> {
        let user_with_id_and_username =
            sqlx::query_as::<_, User>("select * from users where user_id = $1 and username = $2")
                .bind(user_id)
                .bind(username)
                .fetch_optional(&*self.pool)
                .await?;

        Ok(user_with_id_and_username)
    }

    /// Fetches a user that holds the provided invite code
    pub async fn get_by_invite_code(
        &self,
        invite_code: &str,
    ) -> Result<Option<PublicUser>, TalliiError> {
        let user_with_invite_code =
            sqlx::query_as::<_, PublicUser>("select * from users where invite_code = $1")
                .bind(invite_code)
                .fetch_optional(&*self.pool)
                .await?;

        Ok(user_with_invite_code)
    }

    /// Creates a user
    pub async fn create(
        &self,
        new_user: &NewUser,
        crypto: &Crypto,
    ) -> Result<PublicUser, TalliiError> {
        // hash the password
        let hashed_password = crypto.hash_password(&new_user.password).await?;

        // create the user and return the public user
        let user = sqlx::query_as::<_, PublicUser>(
            "insert into users (email, password, invite_code, username) values ($1, $2, $3, $4) returning user_id, avatar, email, username, taunt, verified",
        )
            .bind(&new_user.email)
            .bind(hashed_password)
            .bind(&new_user.invite_code)
            .bind(&new_user.username)
            .fetch_one(&*self.pool)
            .await?;

        Ok(user)
    }
}
