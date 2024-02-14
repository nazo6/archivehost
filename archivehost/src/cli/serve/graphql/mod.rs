use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

mod query;

pub fn router() -> Router {
    let schema = Schema::build(query::QueryRoot, EmptyMutation, EmptySubscription).finish();

    #[cfg(debug_assertions)]
    std::fs::write("../schema.graphql", schema.sdl()).unwrap();

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
