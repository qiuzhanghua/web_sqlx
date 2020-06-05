#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate actix_web;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::prelude::*;
use sqlx::{MySqlPool, PgPool, Pool};
use std::env;

#[cfg(feature = "with-mysql")]
type TdfPool = MySqlPool;
#[cfg(feature = "with-postgres")]
type TdfPool = PgPool;

lazy_static! {
    pub static ref DATABASE_URL: String = {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool: TdfPool = Pool::builder()
        .max_size(40)
        .min_size(40)
        .build(&DATABASE_URL)
        .await
        .unwrap();
    // let recs = sqlx::query!(r#"SELECT * from people"#)
    //     .fetch_all(&mut &pool)
    //     .await
    //     .unwrap();
    // for rec in recs {
    //     println!("{:?}", rec);
    // }

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

pub async fn index(_request: HttpRequest, pool: web::Data<TdfPool>) -> impl Responder {
    let mut cursor = sqlx::query(r#"SELECT * from people"#).fetch(pool.get_ref());
    let row = cursor.next().await.unwrap().unwrap();
    row.get::<&str, &str>("person_id").to_string()
}
