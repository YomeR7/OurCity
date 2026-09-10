mod api_utils;
mod ask_for_construct;
mod get_our_city;
mod vote_for_construct;

/// Register all api method for v1
pub fn register_api(
    router: axum::Router<std::sync::Arc<crate::state::State>>,
) -> axum::Router<std::sync::Arc<crate::state::State>> {
    let router = router.route("/api/get_our_city", axum::routing::get(get_our_city::get_our_city_handler));
    let router = router.route(
        "/api/ask_for_construct",
        axum::routing::post(ask_for_construct::ask_for_construct_handler),
    );
    let router = router.route(
        "/api/vote_for_construct",
        axum::routing::post(vote_for_construct::vote_for_construct_handler),
    );
    router
}
