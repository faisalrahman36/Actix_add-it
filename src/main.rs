use actix_web::{web, App, HttpResponse, HttpServer, Responder,Result};


async fn index_text() -> impl Responder {
    HttpResponse::Ok().body("Hello world is working. Congratulations!")
}

async fn index_html(num: web::Path<u32>) -> Result<HttpResponse> {
    //web::Path type to  string and string to num then add 10
    //let ans = format!("Answer is {}", (MyObj.num.to_string().parse::<i32>().unwrap() + 10).to_string());
    let ans = format!("Answer is {}", (num.to_string().parse::<i32>().unwrap() + 10).to_string());

   Ok(HttpResponse::Ok().body(ans.to_string()))

    //HttpResponse::Ok().json(format!("<html><h1>Answer after adding to initial number is: </h1> <p style='color:blue'> {} </p></html>",ans).to_string());
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index_text))
            .route("/{number}", web::get().to(index_html))
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}