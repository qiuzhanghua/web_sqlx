#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate actix_web;

use actix_web::{middleware, web, App, HttpServer, HttpRequest};
use dotenv::dotenv;
use std::env;
use sqlx::prelude::*;
use sqlx::{MySqlPool, Pool, MySqlConnection, PgPool, PgConnection};


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

    //   let pool: PgPool = Pool::new(&DATABASE_URL).await?;
    let pool: TdfPool = Pool::new(&DATABASE_URL).await.unwrap();
    let recs = sqlx::query!(r#"SELECT * from people"#)
        .fetch_all(&mut &pool)
        .await.unwrap();
    for rec in recs {
        println!("{:?}", rec);
    }

    HttpServer::new(move || {
        App::new()
            // .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(web::resource("/").route(web::get().to(index)))
            // .service(sec::handlers::login)
            // .service(sec::handlers::logout)
            // .service(sec::handlers::me)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await

}

pub async fn index(_pool: web::Data<TdfPool>) -> &'static str {
    use sqlx_core::cursor::Cursor;

    let pool: TdfPool = Pool::new(&DATABASE_URL).await.unwrap();
    let mut cursor = sqlx::query(r#"SELECT * from people"#)
        .fetch(&pool);
    let row = cursor.next().await.unwrap().unwrap();
    println!("{:?}", row.get::<&str, &str>("person_id"));
    "Hello world!"
}
