#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request};
use rocket::response::content::RawHtml;
use serde::Serialize;

#[derive(Serialize)]
struct ClientInfo {
    ip_address: Option<String>,
    user_agent: Option<String>,
    cloudflare_ip: Option<String>,
    cookies: Vec<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientInfo {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let ip_address = req.client_ip().map(|ip| ip.to_string());
        let user_agent = req.headers().get_one("User-Agent").map(|s| s.to_string());
        let cloudflare_ip = req
            .headers()
            .get_one("CF-Connecting-IP")
            .map(|s| s.to_string());

        let cookies = req
            .cookies()
            .iter()
            .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
            .collect();

        Success(ClientInfo {
            ip_address,
            user_agent,
            cloudflare_ip,
            cookies,
        })
    }
}

#[get("/")]
fn index(client_info: ClientInfo) -> RawHtml<String> {
    let ip_address = client_info
        .ip_address
        .unwrap_or_else(|| "Unknown IP Address".to_string());
    let user_agent = client_info
        .user_agent
        .unwrap_or_else(|| "Unknown User Agent".to_string());
    let cloudflare_ip = client_info
        .cloudflare_ip
        .unwrap_or_else(|| "Unknown Cloudflare IP".to_string());
    let cookies = format!("{:?}", client_info.cookies);

    let html_content = format!(
        "<!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"UTF-8\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            <title>Client Info</title>
            <link rel=\"stylesheet\" href=\"https://unpkg.com/missing.css@1.1.1/dist/missing.min.css\">
        </head>
        <body>
            <header>
            </header>
            
            <main>
                <h1>Client Information</h1>
                <p>IP Address: {:?}</p>
                <p>User Agent: {:?}</p>
                <p>Cloudflare IP: {:?}</p>
                <p>Cookies: {:?}</p>
            </main>

            <footer>
            </footer>
        </body>
        </html>",
        ip_address, user_agent, cloudflare_ip, cookies
    );

    RawHtml(html_content)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/css", FileServer::from("static/css"))
}
