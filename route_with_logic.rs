

use axum::{
    body::Bytes, extract::{Json, Path, Query, Request, State}, http::{HeaderMap, StatusCode}, routing::{get, post}, Router
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// use std::collections::BTreeMap;
use sqlx::{pool, postgres::PgPoolOptions};
use tracing_subscriber::field::debug;
use std::{env, error::Error, io::{BufRead, Read}};
use dotenv::dotenv;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};
use anyhow::{ Result};
use jsonwebtoken::{decode, encode, jwk, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use tokio::{
    fs::{File, OpenOptions},
    io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
};

use std::path::Path as BOB;




#[derive(Deserialize)]
pub struct CreateUser {
    email: String,
    password: String,
}

pub async  fn rr() -> String{
    let bob : String = String::from("Root");
    bob
}

pub async  fn fun() -> String{
    let bob : String = String::from("fun");
    println!("fun");

    bob
}

pub async fn post_fun(Json(user): Json<CreateUser>)  -> String{
    let mut  numb: i32 = 0;

    let bob : String = String::from("Post_fun");
    numb += 1;
    println!("Post fun");
    print!("{numb}");
    println!("Email : {} Password : {}",user.email, user.password);
    bob
}
#[derive(Deserialize)]
pub struct  MathThingy {
    first_num : isize,
    second_num : isize,
}

#[derive(Deserialize)]
pub struct  User{
    num1: isize,
    num2: isize,
}
// goal is to have  a route where you can add 2 numbers together 

pub async  fn math_thingy(Json(number): Json<MathThingy>) -> Json<isize>{

    
    let first_num = number.first_num;
    let secound_num = number.second_num;

    let result =     first_num + secound_num;
    Json(result)
}
pub async  fn simple_math(Path(User{num1,num2 }) : Path<User>) -> Json<i128>{
    if num1 == 139 || num2 == 139{
        return Json(3139207761732068657265);
    }

    println!("reacherd");
    let result = num1 - num2;
    println!("{result}");
    Json(result as i128)
}
#[derive(Deserialize)]
pub struct Usercheck {
    username: String,
    password: String,
    email_addres: String,
    age: u8
}

pub async  fn user_check(Json(user): Json<Usercheck>) -> Json<(String)>{
    
    if user.username.is_empty() || user.password.is_empty() || user.email_addres.is_empty()  {
       return axum::Json(Json(json!("didn't suply with a username or a password or an email address ")).to_string());
    }

    let username = user.username;
    let password = user.password;
    let email_address = user.email_addres;
    let age = user.age;

    if age <= 18 {
        return axum::Json(Json(json!("You must be old enough to use this produkt 18+ ")).to_string());

    }
    println!("{username},{password}, {email_address} , {age}");

    Json({
        String::from("Done")
    })
}

// todays task is to have a simple register function and give user a jsonwebtoken and bycrypt their password
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // subject, e.g. username or user_id
    exp: usize,  // expiration timestamp
}


#[derive(Deserialize)]
pub struct UserRegister{
    username: String,
    password: String,
    email_address: String,

}
const MARK: char = '?';                // marker char
const DAY: Duration = Duration::from_secs(60 * 60 * 24);
pub async  fn user_register(Json(user): Json<UserRegister>) -> Json<(String)>{


    let username = user.username;
    let password = user.password;
    let email_address = user.email_address;

    if email_address.len() <= 3 {
        return axum::Json(Json(json!("This email address is not valid brother ")).to_string());

    }

    if username.len() <= 3 && username.len() >= 18 {
        return axum::Json(Json(json!("{username} is to short should be  betwen 3 and 18 caracters ")).to_string());
    }
    if password.len() <= 3 && password.len() >= 18 {
        return axum::Json(Json(json!("{password} is to short should be  betwen 3 and 18 caracters ")).to_string());
    }

    let hash_password: String = hash_password(password).await.expect("idk");
    
    // let hash_password = hash_password.to_string();
    let finalmsg = "Congrats you have made it ";
    let finalmsg2 = "Your password encrypted safely";
    let combined_msg = [finalmsg,finalmsg2,hash_password.as_str()].join(" ");



    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");



    match read_user_by_name(&pool, &username.trim()).await {
        Ok(existing_users) => {
            if !existing_users.is_empty() {
                return axum::Json("Username already exists".to_string());
            }
        },
        Err(e) => {
            eprintln!("DB read failed: {:?}", e);
        }
    }
    match read_user_by_email(&pool, &email_address.trim()).await {
        Ok(existing_users) => {
            if !existing_users.is_empty() {
                return axum::Json("Email already exists".to_string());
            }
        },
        Err(e) => {
            eprintln!("DB read failed: {:?}", e);
        }
    }
    
    match create_user(&pool, &username.trim(), &email_address.trim(),&hash_password.trim()).await {
        Ok((user_id,Token)) => {
            println!("✅ SUCCESS: User created with ID: {} and token {}", user_id,Token);
            Json(format!("User created successfully with ID: {} and token {}", user_id,Token));
        },
        Err(e) => {
            eprintln!("❌ DB insert failed: {:?}", e);
            Json("Failed to create user".to_string());
        }
    }
    Json({
        
        combined_msg
    })
}


pub async  fn decode_user(Json(user): Json<BUSER>) -> Json<(String)>{
    let name = user.name;
    let email_address = user.email;
    let token = user.token;
    let secret = env::var("JWT_SECREAT").expect("JWT_SECREAT must be set");

    // let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secreat.as_ref())).unwrap();
    // let tokenm = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).unwrap();
   match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => {
            let msg = format!("Token valid for user: {}, exp: {}", data.claims.sub, data.claims.exp);
          return  axum::Json(msg.to_string());
        }
        Err(e) => {
            eprintln!("Token decode failed: {:?}", e);
          return  axum::Json("Invalid token".to_string());
        }
    }
  
}

pub async  fn jwt_is_valid(jsonwebtoken:&str) -> Result<bool>{
    let secret = env::var("JWT_SECREAT").expect("JWT_SECREAT must be set");
    match decode::<Claims>(
        &jsonwebtoken,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => {
            let msg = format!("Token valid for user: {}, exp: {}", data.claims.sub, data.claims.exp);
          Ok(true)
        }
        Err(e) => {
            eprintln!("Token decode failed: {:?}", e);
            Ok(false)

        }
    }
  
}




#[derive(Debug, Serialize, Deserialize)]
pub struct NormalRequest{
    token:String
}

pub async  fn  read_whole_thing_and_submit(file_path: impl AsRef<BOB>) -> Result<String, std::io::Error>{
    let path = file_path;
    let mut file = File::open(path).await?;
    let mut empty_string = String::new();
    file.read_to_string(&mut empty_string).await?;
    Ok(empty_string)
}

pub async fn next_daily_message(file: impl AsRef<BOB>) -> io::Result<Option<String>> {
    let path: &BOB = file.as_ref();


    let mut raw = String::new();
    File::open(path).await?.read_to_string(&mut raw).await?;
    let mut lines: Vec<String> = raw.lines().map(|l| l.to_owned()).collect();


    let mut last_reveal = 0u64;
    for l in &lines {
        if let Some((_, rest)) = l.split_once(MARK) {
            if let Some((ts_str, _)) = rest.split_once(MARK) {
                if let Ok(ts) = ts_str.parse::<u64>() {
                    last_reveal = last_reveal.max(ts);
                }
            }
        }
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs();
    if now - last_reveal < DAY.as_secs() {
        return Ok(None);   
    }

    for line in &mut lines {
        if !line.contains(MARK) {                  
            let original = line.clone();
            line.push(MARK);
            line.push_str(&now.to_string());
            line.push(MARK);

            // persist file
            let mut out = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(path)
                .await?;
            out.write_all(lines.join("\n").as_bytes()).await?;
            out.flush().await?;

            return Ok(Some(original));
        }
    }

    Ok(None) 
}

pub async  fn daily_messages(Json(user): Json<NormalRequest>)->(StatusCode,Json<String>){


    let jwt = user.token;
    match jwt_is_valid(&jwt).await {
        Ok(result) =>{
            if result == false{
                println!("Jwt Verify Failed Access Denied");
                return (StatusCode::UNAUTHORIZED, // 401 the status code
                     axum::Json("Verify Failed Access Denied you been loged out try logging in again".to_string()));
            }else{
                let mut passed_ :bool = false;
                let mut resultmsg = String::from("ACCES GRANTED");
                let love_msg = match next_daily_message("lovemsg.txt").await{
                    Ok(Some(s)) => {
                        passed_= true;
                        s},
                    Ok(None) => {
                        
                        passed_ = false;
                        "No new message yet".to_string()
                },
    
                    Err(e) =>{
                        eprintln!("An Error accured of {e} FIX IT");
                        ("No meesages avalable".to_string())
                    }
                };





                println!("Jwt Verify Passed Access granted");
                return (
                   // standart 200
                    if passed_== true {
                        resultmsg = String::from("ACCES GRANTED");
                        StatusCode::OK
                    } else{
                        resultmsg = String::from("ACCES DENIED");
                        StatusCode::UNAUTHORIZED

                    },
                    
                            //HERE LOGIC TO SEND A MESSAGE DAILY TO IT AND THEN PUT THAT MSG THERE FOR 24h and then after 24h delete it and put a new one out so next line 
                            // TO DO LEARN HOW TO READ FROM A FILE AND THEN RETURN THE FILE CONTENTS OF IT
                    axum::Json(json!({"Main": resultmsg, "Lovemsg": love_msg}).to_string()));
                    //  axum::Json(json!({ "main":"Verify  Passed Access granted"});



            }
        }
        Err(e) =>{
            println!("An error accoured please contact the developer of this site thank you so much {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR, // 500 
                 axum::Json(format!("An error accoured please contact the developer of this site thank you so much -> Error : {e}")))
            ;
        }
    }




}






















// pub async  fn decode_tokken(token:String) -> String{
//     let secreat = env::var("JWT_SECREAT").expect("JWT_SECREAT must be set");

// //     let expiration = SystemTime::now()
// //     .checked_add(Duration::from_secs(24 * 3600)) // 24 hours
// //     .unwrap()
// //     .duration_since(UNIX_EPOCH)
// //     .unwrap()
// //     .as_secs() as usize;
// // // sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
// // // .bind(name)
// // // .bind(email)
// // // .execute(pool)
// // // .await?;
// // let my_claims = Claims {
// //     sub: name.to_owned(),
// //     exp: expiration, // unix timestamp
// // };
    

// }
pub async  fn email_update(Json(user): Json<BUSER>) ->  Json<String>{

    let username = user.name;
    let email_address = user.email;

    if email_address.len() <= 3 {
        return axum::Json(axum::Json("This email address is not valid brother ").to_string());
    }

    if username.len() <= 3 && username.len() >= 18 {
        return axum::Json(Json(("{username} is to short should be  betwen 3 and 18 caracters ")).to_string());
    };
  

   match update_mail(&username,&email_address).await{
    Ok((bool)) => {
        println!("✅ Succes users email udated: {}", bool);
        return axum::Json(Json(json!("Updated the email  ")).to_string());

    }
    Err(e) =>{
        println!("Error found couldnt upadte users email {e}");
        return axum::Json(Json(json!("Couldn't update the email {e}  ")).to_string());

    }
   }

   
  
}

async fn hash_password(password: String) -> Result<String>{


    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)

} 

pub async  fn login_user(Json(user): Json<BUSER>) -> Json<String>{

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");
let username = user.name;
let password = user.password;

    match  verify_pass(&username, &password).await {
        Ok(result) => {
            if result == true{
                println!("GOOD JOB USER LOGED IN  -> debug log -> {result}");
                return axum::Json(Json(json!("You logged in hello user  {e}  ")).to_string());

            } else {
                println!("bad JOB USER couldn't log IN  -> debug log -> {result}");
                return axum::Json(Json(json!("Couldn't logg in user  {e}  ")).to_string());

            }
        }
        Err(e) =>{
            println!("Error accured of {e}");
            return axum::Json(Json(json!("Couldn't logg in user  {e}  ")).to_string());

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
    Ok(password)
}

pub async  fn  verify_pass(name:&str,password: &str)-> Result<bool>{

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
}
pub async  fn find_user_data(Json(user): Json<BUSER>) ->  Json<String>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");

    let username = user.name;
    match find_user_fullpool(&pool,&username,).await{
        Ok((user)) => {
            match read_user(&pool, 42).await {
                Ok(Some(u)) =>{ 
                    println!("user found: {:?}", u)

            },
                Ok(None)    =>{
                    
                     println!(" no user with that id");
                     return axum::Json(Json(json!({"result": "no user with that id", "User":user})).to_string());
                },
                Err(e)      => {eprintln!("database error: {e}"
             

            )},
            }

            

            println!(" Succes user data is : {:?}",user );
            return axum::Json(Json(json!({"result": "your user data is succesfuly retrived", "User":user})).to_string());
    
        }
        Err(e) =>{
            println!("Error found couldnt  users  {e}");
            return axum::Json(Json(json!("Couldn't find user  {e}  ")).to_string());
    
        }
}
}



pub     async  fn connect_database() ->Result<()>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");



    println!("Connected to database");

    // println!("{ress:?}");  
    Ok(()  )
}

#[derive(sqlx::FromRow, Deserialize, Debug,Serialize)]
pub struct BUSER {
    #[serde(default)]          // id will be 0 when absent
    pub id:    i32,
    pub name:  String,
    pub email: String,
    pub password: String,
    pub token : String,

}


pub async fn create_user(pool: &sqlx::PgPool, name: &str, email: &str,password: &str) -> Result<(i32,String), sqlx::Error> {


    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(24 * 3600)) // 24 hours
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    // sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
    // .bind(name)
    // .bind(email)
    // .execute(pool)
    // .await?;
    let my_claims = Claims {
        sub: name.to_owned(),
        exp: expiration, // unix timestamp
    };
    let secreat = env::var("JWT_SECREAT").expect("JWT_SECREAT must be set");

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secreat.as_ref())).unwrap();
println!("{password}");
    let user_id = sqlx::query_scalar::<_, i32>("INSERT INTO users (name, email,password) VALUES ($1, $2, $3) RETURNING id")
    .bind(name)
    .bind(email)
    .bind(password)
    .fetch_one(pool)
    .await?;
println!("User created with ID: {}, name: {}, email: {}, hased password {}", user_id, name, email,password);

    println!("user of {name} {email} created ");



    Ok((user_id,token))



}

pub async fn delete_user_main(Json(req): Json<DeleteReq>) -> Json<Value> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    match delete_user(&pool, req.id).await {
        Ok(true)  => Json(json!({ "status": "ok",    "message": "user deleted" })),
        Ok(false) => Json(json!({ "status": "error", "message": "user not found"})),
        Err(e)    => Json(json!({ "status": "error", "message": format!("db error: {e}") })),
    }
}


pub async  fn update_mail( name: &str, email: &str) -> Result<(bool), sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");

match read_user_by_email(&pool, email).await {
    Ok(users) => {
        if users.is_empty() {
            println!("No users found with email: {}", email);
        } else {
            println!("Found {} user(s) with email '{}':", users.len(), email);
            for user in users {
                println!("  - ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
            }
        }
    },
    Err(e) => {
        eprintln!("Error finding user by email: {:?}", e);
    }
}
  let result =   update_user_email(&pool,name,email).await?;


  if result == false{
    Ok((false))

  } else{
    Ok((true))

  }



}



pub async  fn read_user(pool:&sqlx::PgPool,user_id:i32) -> Result<Option<BUSER>, sqlx::Error>{

    let user = sqlx::query_as::<_,BUSER>("SELECT * FROM users WHERE id = $1")
    .bind(user_id)
    .fetch_optional(pool)
    .await?;


    Ok(user)

}

pub async  fn read_user_by_name(pool:&sqlx::PgPool,name:&str) -> Result<Vec<BUSER>, sqlx::Error>{

    let user = sqlx::query_as::<_,BUSER>("SELECT * FROM users WHERE name = $1")
    .bind(name)
    // .fetch_one(pool)
    .fetch_all(pool)    
    .await?;

 

    Ok(user)

}

pub async  fn read_user_by_email(pool:&sqlx::PgPool,email:&str) -> Result<Vec<BUSER>, sqlx::Error>{
    let user = sqlx::query_as::<_,BUSER>("SELECT * FROM users WHERE email = $1")
    .bind(email)
    // .fetch_one(pool)
    .fetch_all(pool)    
    .await?;

 

    Ok(user)

}
async fn find_user_by_email(pool: &sqlx::PgPool, email: &str) {
    match read_user_by_email(pool, email).await {
        Ok(users) => {
            if users.is_empty() {
                println!("No users found with email: {}", email);
            } else {
                println!("Found {} user(s) with email '{}':", users.len(), email);
                for user in users {
                    println!("  - ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
                }
            }
        },
        Err(e) => {
            eprintln!("Error finding user by email: {:?}", e);
        }
    }
}


async fn find_user_by_name(pool: &sqlx::PgPool, name: &str) {
    match read_user_by_name(pool, name).await {
        Ok(users) => {
            if users.is_empty() {
                println!("No users found with name: {}", name);
            } else {
                println!("Found {} user(s) with name '{}':", users.len(), name);
                for user in users {
                    println!("  - ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
                }
            }
        },
        Err(e) => {
            eprintln!("Error finding user by name: {:?}", e);
        }
    }
}


pub async fn delete_user(pool: &sqlx::PgPool, user_id: i32) -> Result<(bool), sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
    .bind(user_id)
    .execute(pool)
    .await?;

Ok(result.rows_affected() > 0)
}
pub async fn update_user_email(pool: &sqlx::PgPool, name: &str, new_email: &str) -> Result<(bool), sqlx::Error> {

   let result =  sqlx::query("UPDATE users SET email = $1 WHERE name = $2")
        .bind(new_email)
        .bind(name)
        .execute(pool)
        .await?;
    
    Ok((result.rows_affected() > 0))
}

pub async  fn find_user_fullpool(pool : &sqlx::PgPool, name: &str)-> Result<Option<BUSER>, sqlx::Error>{ // Vec<Buser> was here before
    let users =  sqlx::query_as::<_, BUSER>("SELECT * FROM users WHERE name = $1")
    .bind(name)
    .fetch_optional(pool)
    .await?;

    Ok(users)
}


#[derive(Deserialize)]
pub struct DeleteReq {
    id: i32,
}





pub async fn get_all_messages(Json(user): Json<NormalRequest>) -> (StatusCode, Json<String>) {
    let jwt = user.token;

    match jwt_is_valid(&jwt).await {
        Ok(valid) => {
            if !valid {
                println!("Jwt Verify Failed Access Denied");
                return (
                    StatusCode::UNAUTHORIZED, // 401
                    Json("Verify Failed Access Denied you been loged out try logging in again".to_string()),
                );
            }

            let content: String = match read_whole_thing_and_submit("lovemain.txt").await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed reading lovemain.txt: {e}");
                   
                    String::new()
                }
            };

            let resultmsg = "ACCES GRANTED".to_string();
            let body = json!({
                "Main": resultmsg,
                "Lovemsg": content
            })
            .to_string();

            (StatusCode::OK, Json(body))
        }

        Err(e) => {
            eprintln!("JWT validation error: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal error during token validation".to_string()),
            )
        }
    }
}
