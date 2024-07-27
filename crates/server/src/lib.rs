pub mod config;
pub mod context;
pub mod graphql;
pub mod handlers;
pub mod services;

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Result;
use async_graphql::{EmptySubscription, Schema};
use handlers::router;
use tokio::net::TcpListener;

use crate::context::Context;
use crate::graphql::schema::{MutationRoot, QueryRoot};

pub async fn start() -> Result<()> {
    let config = config::Config::new();
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    let tcp_listener = TcpListener::bind(addr).await?;
    let context = Context::new(&config).await?;
    let context = Arc::new(context);
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(Arc::clone(&context))
    .finish();
    let app = router(context, schema);

    tracing::info!("GraphQL Playground available on http://{}/graphql", addr);

    axum::serve(tcp_listener, app.into_make_service()).await?;

    Ok(())
}
