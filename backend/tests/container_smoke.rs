//! Container smoke tests for `pulse`.
//!
//! Run against a live container:
//!   SMOKE_PORT=<port> cargo test --test container_smoke -- --ignored --nocapture

use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

const APP_NAME: &str = "pulse";
const DEFAULT_PORT: u16 = 4406;

const FAVICON_CANDIDATES: &[&str] = &["/favicon.png", "/favicon.svg", "/assets/favicon.png"];
const MANIFEST_CANDIDATES: &[&str] = &["/manifest.json", "/assets/manifest.json"];
const CONFIG_CANDIDATES: &[&str] = &["/api/config", "/api/auth/config"];
const SERVICE_WORKER_CANDIDATES: &[&str] = &[
    "/service-worker.js",
    "/api/service-worker.js",
    "/assets/service-worker.js",
];

fn port() -> u16 {
    std::env::var("SMOKE_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_PORT)
}

fn base_url() -> String {
    format!("http://127.0.0.1:{}", port())
}

fn client() -> Client {
    Client::builder()
        .cookie_store(true)
        .timeout(Duration::from_secs(10))
        .build()
        .expect("reqwest client")
}

async fn wait_for_health() {
    let c = client();
    for _ in 0..30 {
        if let Ok(r) = c.get(format!("{}/health", base_url())).send().await {
            if r.status().is_success() {
                return;
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    panic!("container at {} never became healthy", base_url());
}

async fn try_paths(c: &Client, paths: &[&str]) -> Option<reqwest::Response> {
    for p in paths {
        if let Ok(r) = c.get(format!("{}{}", base_url(), p)).send().await {
            if r.status().is_success() {
                return Some(r);
            }
        }
    }
    None
}

// ---------- common tests ----------

#[tokio::test]
#[ignore]
async fn health_returns_200() {
    let c = client();
    let r = c.get(format!("{}/health", base_url())).send().await.unwrap();
    assert_eq!(r.status(), 200, "expected 200 from /health");
}

#[tokio::test]
#[ignore]
async fn root_serves_html() {
    let c = client();
    let r = c.get(&base_url()).send().await.unwrap();
    assert_eq!(r.status(), 200, "expected 200 from /");
    let ct = r
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(ct.starts_with("text/html"), "expected text/html, got {ct:?}");
}

#[tokio::test]
#[ignore]
async fn favicon_resolves() {
    let c = client();
    let r = try_paths(&c, FAVICON_CANDIDATES)
        .await
        .unwrap_or_else(|| panic!("no favicon path returned 2xx: {FAVICON_CANDIDATES:?}"));
    let ct = r
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        ct.starts_with("image/") || ct.starts_with("application/octet-stream"),
        "expected image/* (or octet-stream), got {ct:?}"
    );
}

#[tokio::test]
#[ignore]
async fn manifest_parses_as_pwa() {
    let c = client();
    let r = try_paths(&c, MANIFEST_CANDIDATES)
        .await
        .unwrap_or_else(|| panic!("no manifest path returned 2xx: {MANIFEST_CANDIDATES:?}"));
    let v: Value = r.json().await.unwrap();
    assert!(v["name"].is_string(), "manifest.name must be a string, got {v:?}");
    assert!(v["icons"].is_array(), "manifest.icons must be an array");
}

#[tokio::test]
#[ignore]
async fn config_endpoint_has_site_title() {
    let c = client();
    let r = try_paths(&c, CONFIG_CANDIDATES)
        .await
        .unwrap_or_else(|| panic!("no config path returned 2xx: {CONFIG_CANDIDATES:?}"));
    let v: Value = r.json().await.unwrap();
    let title = v["siteTitle"]
        .as_str()
        .or_else(|| v["site_title"].as_str())
        .unwrap_or("");
    assert!(
        title.eq_ignore_ascii_case(APP_NAME),
        "expected siteTitle == {APP_NAME:?}, got {title:?}"
    );
}

#[tokio::test]
#[ignore]
async fn service_worker_or_frontend_serves() {
    let c = client();
    let r = try_paths(&c, SERVICE_WORKER_CANDIDATES).await;
    assert!(
        r.is_some(),
        "no service-worker path returned 2xx: {SERVICE_WORKER_CANDIDATES:?}"
    );
}

// ---------- per-app tests: pulse (Prometheus metrics) ----------

#[tokio::test]
#[ignore]
async fn metrics_returns_prometheus_text_format() {
    wait_for_health().await;
    let c = client();

    let r = c
        .get(format!("{}/api/metrics", base_url()))
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), 200, "expected 200 from /api/metrics");

    let ct = r
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    assert!(
        ct.starts_with("text/plain"),
        "Prometheus exposition must be text/plain, got {ct:?}"
    );

    let body = r.text().await.unwrap();
    assert!(
        body.contains("# HELP") || body.contains("# TYPE"),
        "metrics body lacks Prometheus exposition comments: {body:?}"
    );
    assert!(
        body.contains("process_") || body.contains("cpu"),
        "metrics body lacks any process_* or cpu metric: {body:?}"
    );
}
