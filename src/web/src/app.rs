use http::HeaderValue;
use tide::middleware::{Cors, Origin};
use tide::{Server, IntoResponse, Response};
use crate::controller::sensitive;

pub fn result_to_response<T: IntoResponse, E: IntoResponse>(r: Result<T, E>) -> Response {
    match r {
        Ok(r) => r.into_response(),
        Err(r) => {
            let res = r.into_response();
            if res.status().is_success() {
                panic!(
                    "Attempted to yield error response with success code {:?}",
                    res.status()
                )
            }
            res
        }
    }
}

pub fn get_app() -> Server<()> {
    let mut app = Server::new();
    app = add_middleware(app);
    app = add_routes(app);
    app
}

pub fn add_middleware<State: 'static + Sync + Send>(mut app: Server<State>) -> Server<State> {
    let rules = Cors::new()
        .allow_methods(HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"))
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    app.middleware(tide::middleware::RequestLogger::new());
    app.middleware(rules);
    app
}

pub fn add_routes(mut api: Server<()>) -> Server<()> {
    api.at("/v1/sensitive/filter/")
        .get(sensitive::filter);

    // api.at("/v1/sensitive/:id")
    //     .get(|req| async move { result_to_response(crate::articles::get_article(req).await) })
    //     .put(|req| async move { result_to_response(crate::articles::update_article(req).await) })
    //     .delete(
    //         |req| async move { result_to_response(crate::articles::delete_article(req).await) },
    //     );
    api
}