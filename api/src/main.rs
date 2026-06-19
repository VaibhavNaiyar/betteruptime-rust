use std::sync::{Arc, Mutex};
use poem::{Route, Server, get, handler, listener::TcpListener, post, web::{Data, Json, Path}};

use crate::{request_inputs::{CreateUserInput, CreateWebsiteInput}, request_outputs::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput}};

use store::store::Store;
pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(id): Path<String>, Data(s): Data<Arc<Mutex<Store>>>) -> Json<GetWebsiteOutput> {
    let mut store = s.lock().unwrap();
    let website = store.get_website(id).unwrap();
    Json(GetWebsiteOutput {
        url: website.url
    })
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<Arc<Mutex<Store>>>) -> Json<CreateUserOutput> {
    let mut store = s.lock().unwrap();
    let id = store.sign_up(data.username, data.password).unwrap();
    let response = CreateUserOutput { id };
    Json(response)
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<Arc<Mutex<Store>>>) -> Json<SigninOutput> {
    let mut store = s.lock().unwrap();
    let _exists = store.sign_in(data.username, data.password).unwrap();
    let response = SigninOutput {
        jwt: String::from("vaibhav")
    };
    Json(response)
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>, Data(s): Data<Arc<Mutex<Store>>>) -> Json<CreateWebsiteOutput> {
    let mut store = s.lock().unwrap();
    let website = store.create_website(String::from("dd020379-1e62-44b2-8a3d-c4e17c30d044"), data.url).unwrap();
    let response = CreateWebsiteOutput { id: website.id };
    Json(response)
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::new().unwrap()));
    let app: Route = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("/user/signin", post(sign_in))
        .data(s);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
