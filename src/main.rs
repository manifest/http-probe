use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::start();

    let app = app::run().await?;
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}

mod app;
