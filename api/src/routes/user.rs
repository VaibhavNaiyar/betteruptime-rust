use std::sync::{Arc, Mutex};
use jsonwebtoken::{encode, EncodingKey, Header};
use poem::{Error, handler, http::StatusCode, web::{Data, Json}};
use serde::{Deserialize, Serialize};
use store::store::Store;
use crate::request_inputs::CreateUserInput;
use crate::request_outputs::{CreateUserOutput, SigninOutput};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<Arc<Mutex<Store>>>) -> Result<Json<CreateUserOutput>, Error> {
    let mut store = s.lock().unwrap();
    let id = store.sign_up(data.username, data.password).map_err(|_| Error::from_status(StatusCode::CONFLICT))?;
    let response = CreateUserOutput { id };
    Ok(Json(response))
}

#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<Arc<Mutex<Store>>>) -> Result<Json<SigninOutput>, Error> {
    let mut store = s.lock().unwrap();
    let user_id = store.sign_in(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            let my_claims = Claims {
                sub: user_id,
                exp: 11111111,
            };

            let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secrets".as_ref()))
                .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

            let response = SigninOutput {
                jwt: token,
            };
            Ok(Json(response))
        }
        Err(_) => Err(Error::from_status(StatusCode::UNAUTHORIZED)),
    }
}
