use actix_web::{get, test, web, App, HttpRequest, HttpResponse};

#[get("/user/{id}")]
fn path_segment_param(req: HttpRequest, path: web::Path<(u32,)>) -> HttpResponse {
    // uri: /user/123, path: None, skip: 9, segments: [("id", Segment(6, 9))]
    dbg!(req.clone());
    println!("path.0 = {}", path.0);
    println!("&req.match_info()[\"id\"] = {}", &req.match_info()["id"]);
    println!(
        "req.match_info().query(\"id\") = {}",
        req.match_info().query("id")
    );
    println!(
        "req.match_info().get(\"id\").unwrap() = {}",
        req.match_info().get("id").unwrap()
    );
    HttpResponse::Ok().body("asdf")
}

async fn test_path_segment_param() {
    let mut app_service = test::init_service(App::new().service(path_segment_param)).await;
    let req = test::TestRequest::get().uri("/user/123").to_request();
    test::call_service(&mut app_service, req).await;
}

#[actix_rt::main]
async fn main() {
    test_path_segment_param().await;
}