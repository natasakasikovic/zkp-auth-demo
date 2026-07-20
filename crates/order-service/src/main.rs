mod client;
mod domain;
mod error;
mod handlers;
mod routes;
mod state;

use std::net::SocketAddr;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = AppState::from_env();
    let app = routes::router(state);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("order-service listening on http://{address}");
    axum::serve(listener, app).await?;

    Ok(())
}
