use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn home() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(r#"
        <!DOCTYPE html>
<html lang="en">
    <head>
        <!-- This is equivalent to a HTTP header -->
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Home</title>
    </head>
    <body>
        <p>Welcome to our newsletter!</p>
    </body>
</html>
        "#)
}