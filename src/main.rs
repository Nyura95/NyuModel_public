use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handlers;
mod models;
mod persistance;
mod errors;
mod accounts;
mod jwt_check;
mod rate_limit;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env::var("TOKEN_SECRET").expect("TOKEN_SECRET must be set");
    env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");
    let addr = env::var("ADDR_SERVER").expect("ADDR_SERVER must be set");
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = mysql::Pool::new(url.as_str()).unwrap_or_else(|err| {
        eprintln!("Erreur lors de la connexion à la base de données: {}", err);
        std::process::exit(1);
    });
    let db = web::Data::new(pool);
    let accounts = accounts::AccountDB::new();

    {
        let all_accounts = persistance::accounts::get_account_data(&db).unwrap();
        for account in all_accounts {
            accounts.add_account(account).unwrap_or_else(|err| {
                eprintln!("Erreur lors de la récupération des comptes: {}", err);
                std::process::exit(1);
            });
        }
    }

    println!("🚀 Server started successfully");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("null")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                    ])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(rate_limit::RateLimit::new(10))
            .app_data(web::Data::new(accounts.clone()))
            .app_data(db.clone())
            .service(handlers::healthcheck::health_checker_handler)
            .service(
                web::scope("/v1")
                .service(handlers::accounts::accounts_create_handler)
            )
            .service(handlers::login::login_handler)    
            .service(
                web::scope("/v1")
                    .wrap(jwt_check::JwtCheck)
                    .service(handlers::accounts::accounts_list_handler)
                    .service(handlers::accounts::account_get_handler)
                    .service(handlers::accounts::account_update_handler)
            )
            
    })
    .bind(addr)?
    .workers(2)
    .run()
    .await
}
