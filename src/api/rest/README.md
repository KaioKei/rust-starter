# REST API

## Axum

[Axum](https://github.com/tokio-rs/axum) is based on [Tokio](https://tokio.rs/). It is ergonomic and efficient. 
It is a good choice for a modular and async-friendly approach.

[Axum docs](https://docs.rs/axum/latest/axum/)

[Axum tuto](https://www.shuttle.dev/blog/2023/12/06/using-axum-rust)

Server example :

```rust
use axum::{
    routing::get,
    Router,
};

// which calls one of these handlers
async fn get_foo() {}
async fn post_foo() {}
async fn foo_bar() {}


#[tokio::main]
async fn main() {

    // our router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Actix-web

[Actix-web](https://actix.rs/docs/whatis) is based on [Actix](https://github.com/actix/actix) runtime, which is async-std.
It is very efficient and mature.

[Actix Github](https://github.com/actix/actix-web)

[API doc](https://docs.rs/actix-web/latest/actix_web/)

```rust
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```