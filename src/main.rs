use actix_web::{App, HttpResponse, HttpServer, Result, web};
use serde::Deserialize;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use tera::{Context, Tera};

#[derive(Deserialize)]
struct FormData {
    height: f64,
    weight: f64,
    text: String,
}

//getの場合はweb::Getにする
async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let rendered = tmpl
        .render("index.html", &Context::new())
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}

//postの場合はweb::Fromにする
async fn calc(
    query: web::Form<FormData>,
    tmpl: web::Data<Tera>,
    db_pool: web::Data<MySqlPool>,
) -> Result<HttpResponse> {
    let h = query.height / 100.0;
    let bmi = query.weight / (h * h);
    let per = (bmi / 22.0) * 100.0;
    let txt = query.text.clone();

    //result.htmlにわたすもの
    let mut context = Context::new();
    context.insert("bmi", &bmi);
    context.insert("per", &per);
    context.insert("text", &txt);

    //DBに登録
    sqlx::query!(
        "INSERT INTO bmi_records (height, weight, bmi, text) VALUES (?, ?, ?, ?)",
        query.height,
        query.weight,
        bmi,
        txt
    )
    .execute(db_pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("DBエラー: {}", e);
        actix_web::error::ErrorInternalServerError("DB挿入失敗")
    })?;

    let rendered = tmpl
        .render("result.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(rendered))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("テンプレート読み込みエラー: {}", e);
            std::process::exit(1);
        }
    };

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URLが見つかりません");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("DB接続失敗");

    println!("http://127.0.0.1:8888");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(pool.clone())) 
            .route("/", web::get().to(index))
            .route("/calc", web::post().to(calc))
        //送信したいメソッドに合わせてpost,getを切り替える
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
