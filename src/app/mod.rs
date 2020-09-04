use anyhow::Result;
use async_std::{prelude::StreamExt, stream, task};
use serde_derive::Deserialize;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, SystemTime},
};
use tide::{
    http::{headers::HeaderValue, StatusCode},
    log::info,
    security::{CorsMiddleware, Origin},
    sse,
};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub(crate) struct State {
    throughput: Arc<AtomicUsize>,
}

#[derive(Debug, Deserialize)]
struct AddRequest {
    value: usize,
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) async fn run() -> Result<tide::Server<State>> {
    let state = State {
        throughput: Arc::new(AtomicUsize::new(0)),
    };

    task::spawn(reset(state.throughput.clone()));

    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);

    let mut app = tide::with_state(state);
    app.with(cors);

    app.at("/").all(index);
    app.at("/sse").get(sse::endpoint(sse_index));

    Ok(app)
}

async fn reset(throughput: Arc<AtomicUsize>) -> Result<()> {
    let mut interval = stream::interval(Duration::from_secs(60));
    while interval.next().await.is_some() {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        info!("{}: {}", now, throughput.swap(0, Ordering::SeqCst));
    }

    Ok(())
}

async fn index(req: tide::Request<State>) -> tide::Result {
    req.state().throughput.fetch_add(1, Ordering::SeqCst);

    Ok(tide::Response::new(StatusCode::Ok))
}

async fn sse_index(req: tide::Request<State>, sender: tide::sse::Sender) -> tide::Result<()> {
    let data = req.state().throughput.load(Ordering::Relaxed).to_string();
    sender.send("throughput", data, None).await?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

#[async_std::test]
async fn post_test() -> Result<()> {
    use tide::http::{Method, Url};

    let app = run().await?;
    let req = tide::http::Request::new(Method::Get, Url::parse("http://localhost:8080")?);
    let mut res: tide::http::Response = app.respond(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::Ok);
    assert_eq!(res.body_string().await.unwrap(), "".to_owned());

    Ok(())
}
