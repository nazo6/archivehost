use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use super::AppState;

mod common;
mod interface;
mod mutation;
mod query;
mod subscription;

pub fn router(state: AppState) -> Router<AppState> {
    let schema = Schema::build(
        query::QueryRoot::default(),
        mutation::MutationRoot,
        subscription::Subscription::default(),
    )
    .data(state)
    .finish();

    #[cfg(debug_assertions)]
    std::fs::write("../schema.graphql", schema.sdl()).expect("Unable to write schema");

    Router::new()
        .route(
            "/graphql",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/graphql/ws", GraphQLSubscription::new(schema))
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .subscription_endpoint("/graphql/ws")
            .finish(),
    )
}
