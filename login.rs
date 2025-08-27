use axum::{
    body::Bytes, extract::{Json, Path, Query, Request, State}, http::HeaderMap, routing::{get, post}, Router
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// use std::collections::BTreeMap;
use sqlx::{pool, postgres::PgPoolOptions};
use tracing_subscriber::field::debug;
use std::{env, io::BufRead};
use dotenv::dotenv;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use anyhow::Result;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
#[derive(Deserialize)]
pub struct CreateUser {
    email: String,
    password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}


#[derive(Deserialize)]
pub struct UserRegister{
    username: String,
    password: String,
    email_address: String,

}

#[derive(sqlx::FromRow, Deserialize, Debug,Serialize)]
pub struct BUSER {
    #[serde(default)]
    pub id:    i32,
    pub name:  String,
    pub email: String,
    pub password: String,
    pub token : String,

}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
    pub user_id: i32,
}
pub async fn login_user(Json(user): Json<LoginRequest>) -> Json<LoginResponse> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let username = user.name;
    let password = user.password;
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");


    match verify_pass(&username, &password,&pool).await {
        Ok(result) => {
            if result {
                let token = generate_jwt_token(&username).await.expect("FAILED TO MAKE TOKEN");
                println!("GOOD JOB USER LOGGED IN -> debug log -> {result}");
                


          

                Json(LoginResponse {
                    message: "Login successful".to_string(),
                    token,
                    user_id: 0, 
                })
            } else {
                println!("bad JOB USER couldn't log IN -> debug log -> {result}");
                Json(LoginResponse {
                    message: "Invalid credentials".to_string(),
                    token: String::new(),
                    user_id: 0,
                })
            }
        }
        Err(e) => {
            println!("Error occurred: {e}");
            Json(LoginResponse {
                message: format!("No User found with that username "),
                token: String::new(),
                user_id: 0,
            })
        }
    }
}
pub async fn find_password_from_name(name: &str, pool: &sqlx::PgPool) -> Result<String, sqlx::Error> {
    use sqlx::Row;
    let row = sqlx::query("SELECT password FROM users WHERE name = $1")
        .bind(name)
        .fetch_one(pool)
        .await?;
    
    let password: String = row.get("password");
    Ok((password))
}

pub async  fn  verify_pass(name:&str,password: &str,poool:&sqlx::PgPool)-> Result<bool>{

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");



    let hashed_pass = find_password_from_name(name,&pool).await?;

    let verify = verify(password,  &hashed_pass)?;
Ok(verify)
}async fn generate_jwt_token(username: &str) -> Result<String> {
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(24 * 3600)) // 24 hours
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration,
    };
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");


    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    update_user_token(&pool, username, &token).await?;

    Ok(token)
}

pub async fn update_user_token(pool: &sqlx::PgPool, name: &str, new_token: &str) -> Result<(bool), sqlx::Error> {

    let result =  sqlx::query("UPDATE users SET token = $1 WHERE name = $2")
         .bind(new_token)
         .bind(name)
         .execute(pool)
         .await?;
     
     Ok((result.rows_affected() > 0))
 }
 
