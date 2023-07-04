mod args;
mod blog;
mod pull_requests;
mod run;
mod sponsors;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run::run().await
}
