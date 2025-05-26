use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse,Error};
use serde::{Deserialize, Serialize};

//アドレスとポートを指定
const SERVER_ADDR: &str = "127.0.0.1:8888";

//Actix Webのメイン関数
//asyncは非同期処理を行う関数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[SERVER] http://{}/", SERVER_ADDR);
    //HTTPサーバー起動
    HttpServer::new(|| {
        //ルーティングを指定
        //routeメソッドで適切に振り分けることでURLごとに処理を振り分けられる
        App::new()
        .route("/", web::get().to(index))
        .route("/calc", web::get().to(calc))
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}

//"/"実行される関数
async fn index(_:HttpRequest) -> Result<HttpResponse, Error> {
    Ok (HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("{}{}{}{}{}{}",
            "<html><body><h1>BMI測定</h1>",
            "<form action='calc'>",
            "身長: <input name='height' value='160'><br>",
            "体重: <input name='weight' value='70'><br>",
            "<input type='submit' value='送信'>",
            "</form></body></html>")))
}

//入力フォームの定義
#[derive(Serialize, Deserialize, Debug)]
pub struct FromBMI {
    height: f64,
    weight: f64,
}

//"/calc"にアクセスされたときに実行
async fn calc(q: web::Query<FromBMI>) -> Result<HttpResponse, Error> {
    //フォームからちゃんとパラメーターを受け取ったか確認
    println!("{:?}", q);
    //BMIを計算
    let h = q.height / 100.0;
    let bmi = q.weight / (h * h);
    let per = (bmi / 22.0) * 100.0;
    //結果を表示
    Ok (HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "<h3>BMI={:.1}, 乖離率={:.1}%</h3>", bmi, per)
        )
    )
}