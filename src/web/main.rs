use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use auditor_lib::auditor::Auditor;
use auditor_lib::config::Config;
use auditor_lib::errors::AuditError;

#[derive(Clone)]
struct AppState {
    auditor: Auditor,
}

#[derive(Deserialize)]
struct AuditRequest {
    text: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum AuditResponse {
    Success,
    Failure { message: String, error: AuditError },
}

#[post("/audit")]
async fn audit(json: web::Json<AuditRequest>, data: web::Data<AppState>) -> impl Responder {
    if let Err(err) = data.auditor.audit(&json.text) {
        let message = "Errors found while auditing text";
        return (
            web::Json(AuditResponse::Failure {
                message: message.to_string(),
                error: err,
            }),
            actix_web::http::StatusCode::BAD_REQUEST,
        );
    }
    (
        web::Json(AuditResponse::Success),
        actix_web::http::StatusCode::OK,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    // env_logger::init();

    // Parse the config
    let config = Config::load_all("config.toml");

    // Construct an auditor
    let auditor = Auditor::new(config.disallowed_strings, config.max_token_length);

    // Construct app state
    let app_state = AppState { auditor };

    // Run server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(audit)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
