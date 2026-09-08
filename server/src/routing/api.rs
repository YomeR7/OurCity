/// Register all api method for v1
pub fn register_api(
    router: axum::Router<std::sync::Arc<crate::state::State>>,
) -> axum::Router<std::sync::Arc<crate::state::State>> {
    router
}
