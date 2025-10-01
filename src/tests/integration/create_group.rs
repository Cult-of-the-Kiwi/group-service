use axum::{body::Body, extract::Request};
use hyper::Method;
use serde_json::json;
use tower::ServiceExt;

use crate::{app, tests::setup::ctx};

#[tokio::test]
async fn test_create_group() -> anyhow::Result<()> {
    let ctx = ctx().await;

    let router = app::build(Some(ctx.config.clone())).await?;

    let jwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjk5OTk5OTk5OTksInVzZXJfaWQiOiIwMDAwMDAwMC0wMDAwLTAwMDAtMDAwMC0wMDAwMDAwMDAwMDAifQ.EXD8n0oJYSizDESJ3K2ml7Q-CgMCJxPGgUh7_job1Ag";
    // let jwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjk5OTk5OTk5OTksInVzZXJfaWQiOiJkYTNiYmI1NC1lMWM1LTQ5YzQtYmZiZi1kNDM5OTA4OTdlMDUifQ.8rk4quWkit5pIDjZYni07hiyzd6OsXGJcLYHiV9k-Fc";

    let body = json!({
        "name": "Test Group",
        "member_ids": [
            "11111111-1111-1111-1111-111111111111",
            "22222222-2222-2222-2222-222222222222"
        ]
    });

    let request = Request::builder()
        .method(Method::POST)
        .uri("/groups")
        .header("Authorization", format!("Bearer {}", jwt))
        .header("Content-Type", "application/json")
        .body(Body::from(body.to_string()))?;

    let response = router.oneshot(request).await?;

    assert_eq!(response.status(), 200);

    Ok(())
}
