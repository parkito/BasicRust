use actix_web::{web, App, HttpServer, HttpResponse};
use serde::Deserialize;

fn main() {
    run_server();
}

#[actix_web::main]
async
fn run_server() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/user", web::post().to(post_user))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/user" method="post">
                <input type="text" name="username"/>
                <input type="text" name="password"/>
                <button type="submit">Register</button>
                </form>
            "#,
        )
}

#[derive(Deserialize)]
struct UserDto {
    username: String,
    password: String,
}

async fn post_user(form: web::Form<UserDto>) -> HttpResponse {
    if form.username.is_empty() || form.password.is_empty() {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Values were not provided");
    }

    let response =
        format!("Provided user: {} password: {}",
                form.username, form.password);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}