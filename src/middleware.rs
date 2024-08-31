
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
    //Extract, Json,
};
use tracing::{info,error};
//use std::time::Instant;

pub async fn log_request(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let start_time = std::time::Instant::now();

    let response = next.run(req).await;

    let duration = start_time.elapsed();
    let status = response.status().as_u16();
    
    info!(
        "method = {}, uri = {}, status = {}, duration = {:.2?}",
        method, uri, status, duration
    );

    if status >= 400 {
        error!(
            "Error response: method = {}, uri = {}, status = {}, duration = {:.2?}",
            method, uri, status, duration
        );
    }

    Ok(response)
}
