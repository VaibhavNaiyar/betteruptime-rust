use std::sync::{Arc, Mutex};
use poem::{Route, Server, get, listener::TcpListener, post};
use store::store::Store;

pub mod request_inputs;
pub mod request_outputs;
pub mod routes;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::new().unwrap()));
    let app: Route = Route::new()
        .at("/website/:website_id", get(routes::website::get_website))
        .at("/website", post(routes::website::create_website))
        .at("/user/signup", post(routes::user::sign_up))
        .at("/user/signin", post(routes::user::sign_in))
        .data(s);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
