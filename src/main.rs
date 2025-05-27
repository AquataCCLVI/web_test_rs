use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::Deserialize;
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
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(rendered))
}

//postの場合はweb::Fromにする
async fn calc(query: web::Form<FormData>, tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let h = query.height / 100.0;
    let bmi = query.weight / (h * h);
    let per = (bmi / 22.0) * 100.0;
    let txt = query.text.clone();

    let mut context = Context::new();
    context.insert("bmi", &bmi);
    context.insert("per", &per);
    context.insert("text", &txt);

    let rendered = tmpl
        .render("result.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(rendered))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").expect("Template loading error");

    println!("http://127.0.0.1:8888");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .route("/", web::get().to(index))
            .route("/calc", web::post().to(calc))
            //送信したいメソッドに合わせてpost,getを切り替える
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
