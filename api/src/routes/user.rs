use std::sync::{Arc, Mutex};
use poem::{handler, web::{Data, Json}};
use store::store::Store;
use crate::request_inputs::CreateUserInput;
use crate::request_outputs::{CreateUserOutput, SigninOutput};

#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<Arc<Mutex<Store>>>) -> Json<CreateUserOutput> {
    let mut store = s.lock().unwrap();
    let id = store.sign_up(data.username, data.password).unwrap();
    let response = CreateUserOutput { id };
    Json(response)
}

#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<Arc<Mutex<Store>>>) -> Json<SigninOutput> {
    let mut store = s.lock().unwrap();
    let _exists = store.sign_in(data.username, data.password).unwrap();
    let response = SigninOutput {
        jwt: String::from("vaibhav")
    };
    Json(response)
}
