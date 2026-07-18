mod domain;
mod error;
mod handlers;
mod routes;
mod state;

use std::net::SocketAddr;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = AppState::new();
    let app = routes::router(state);

    let address = SocketAddr::from(([127, 0, 0, 1], 3002));
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("payment-service listening on http://{address}");
    axum::serve(listener, app).await?;

    Ok(())
}
