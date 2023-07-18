    use crate::models::User;
    use diesel::prelude::*;
    use actix_web::{web, HttpResponse, Responder};
    use bcrypt::verify;
    use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
    use serde::Serialize;

    #[derive(Serialize)]
    struct Token {
        token: String,
    }

    pub async fn signup(user: web::Json<User>, db: web::Data<PoolType>) -> impl Responder {
        use crate::schema::users::dsl::*;

        let connection = db.get().unwrap();
        let new_user = User::new(user.name.clone(), user.email.clone(), user.password.clone());
        diesel::insert_into(users).values(&new_user).execute(&connection).unwrap();

        HttpResponse::Ok().finish()
    }

    pub async fn login(user: web::Json<User>, db: web::Data<PoolType>) -> impl Responder {
        use crate::schema::users::dsl::*;

        let connection = db.get().unwrap();
        let result = users.filter(username.eq(&user.username)).first::<User>(&connection);

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
