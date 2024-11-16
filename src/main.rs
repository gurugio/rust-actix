use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

/*async fn handle_step(req_body: String) -> impl Responder {
    let registers = json!({
        "AX": "0000",
        "BX": "0000"
    });
    let serialized = serde_json::to_string(&registers).unwrap();
    HttpResponse::Ok().body(serialized)
}*/

async fn handle_step(req_body: String) -> impl Responder {
    println!("/step: Receive data={}", req_body);
    HttpResponse::Ok().json({
        // 예시로 간단한 JSON 응답을 반환합니다.
        serde_json::json!({
            "AX": "0000",
            "BX": "0000"
        })
    })
}

async fn handle_reload(req_body: String) -> impl Responder {
    println!("/reload: Receive data={}", req_body);
    HttpResponse::Ok().json({
        // 예시로 간단한 JSON 응답을 반환합니다.
        serde_json::json!({
            "AX": "0000",
            "BX": "0000"
        })
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Rust web-server started at 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    //.allowed_origin("http://127.0.0.1:8080") // 특정 도메인을 허용
                    .allow_any_origin() // 특정 도메인을 허용
                    .allowed_methods(vec!["GET", "POST"]) // 허용할 HTTP 메서드
                    .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                    .max_age(3600),
            )
            .route("/step", web::post().to(handle_step))
            .route("/reload", web::post().to(handle_reload))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
