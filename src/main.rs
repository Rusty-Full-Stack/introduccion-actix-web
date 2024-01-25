use actix_web::{get, http::StatusCode, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hola() -> impl Responder {
    HttpResponse::Ok().body("Hola Mundo!")
}

async fn despedida() -> impl Responder {
    HttpResponse::Ok().body("Adios!")
}

#[get("/formulario")]
async fn formulario() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        <!doctype html>
        <html lang="es">
            <body>
                <form method="post" action="/formulario">
                    <button type="submit">Adios</button>
                </form>
            </body>
        </html>
        "#,
    )
}

#[post("/formulario")]
async fn procesar_formulario() -> impl Responder {
    web::Redirect::to("/despedida").using_status_code(StatusCode::FOUND)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hola)
            .route("/despedida", web::get().to(despedida))
            .service(formulario)
            .service(procesar_formulario)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
