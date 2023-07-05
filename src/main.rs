use std::error::Error;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let url = "https://tempo-prod-15-prod-us-west-0.grafana.net:443/v1/traces";
    let endpoint = tonic::transport::Endpoint::from_static(url);
    endpoint.connect().await?;

    Ok(())
}
