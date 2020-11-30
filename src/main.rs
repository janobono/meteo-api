use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use mysql::*;
use mysql::prelude::*;

mod meteo;

struct AppState {
    pool: Pool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = String::from("127.0.0.1:") + meteo::env("METEO_API_PORT", "8080").as_str();
    println!("Server address {}", addr);

    let db_url = meteo::env("METEO_API_DB_URL", "mysql://app:app@localhost:3306/app");
    println!("Database url {}", db_url.as_str());

    let app_state = web::Data::new(
        AppState {
            pool: Pool::new(db_url).expect("Database connection error!"),
        }
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/api/data", web::get().to(meteo_data))
    })
        .bind(addr)?
        .run()
        .await
}

async fn meteo_data(query_input: web::Query<meteo::QueryInput>, data: web::Data<AppState>) -> impl Responder {
    let (_, sql) = meteo::to_sql(&query_input);
    let con = &mut data.pool.get_conn().expect("Database connection error!");
    let result = meteo::to_json(&mut con.query_iter(sql).unwrap());

    HttpResponse::Ok()
        .content_type("application/json")
        .body(result.to_owned())
}
