// api/src/auth.rs
use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::models::user::User;
use crate::schema::users;
use crate::db::DbPool;

#[derive(Serialize)]
struct Token {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

pub async fn signup(user: web::Json<User>, db: web::Data<DbPool>) -> impl Responder {
    let connection = db.get().unwrap();
    let new_user = User::new(user.username.clone(), user.name.clone(), user.email.clone(), user.password.clone());
    diesel::insert_into(users::table).values(&new_user).execute(&connection).unwrap();

    HttpResponse::Ok().finish()
}

pub async fn login(user: web::Json<User>, db: web::Data<DbPool>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = users::table.filter(users::username.eq(&user.username)).first::<User>(&connection);

    match result {
        Ok(user_from_db) => {
            if verify(&user.password, &user_from_db.password_hash).unwrap() {
                let claims = Claims { sub: user_from_db.username };
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
                HttpResponse::Ok().json(Token { token })
            } else {
                HttpResponse::Unauthorized().finish()
            }
        },
        _ => HttpResponse::InternalServerError().finish(),
    }
}
