use std::time::Duration;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use tokio;
use tokio::time;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let exporter = opentelemetry_otlp::new_exporter().tonic();

    let resource: opentelemetry_sdk::resource::Resource = Resource::new([
        KeyValue::new(SERVICE_NAME, "pxfnc_local_service")
    ]);

    let _tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(opentelemetry_sdk::trace::config().with_resource(resource))
        .install_batch(opentelemetry_sdk::runtime::TokioCurrentThread)
        .expect("install failed");

    let _ = HttpServer::new(|| App::new().wrap(RequestTracing::new()).service(roll_dice))
        .bind("0.0.0.0:8000")?
        .run()
        .await;

    global::shutdown_tracer_provider();
    Ok(())
}

// 適当に遅延させながらサイコロを転がすAPI
#[get("/roll_dice")]
async fn roll_dice() -> impl Responder {
    let num: f32 = rand::random::<f32>() * 6.0;

    time::sleep(Duration::from_millis(2.0f32.powf(num).floor() as u64 * 100)).await;

    let json = serde_json::Value::Number(((num.floor() as i32) + 1).into());
    HttpResponse::Ok().json(json)
}
