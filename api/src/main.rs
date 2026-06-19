use poem::{Route, Server, get, handler, http::response, listener::TcpListener, post, web::{Json, Path}};

use crate::{request_inputs::{CreateUserInput, CreateWebsiteInput}, request_outputs::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput}};

use store::{models::website, store::Store};
pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(id):Path<String>) -> Json<GetWebsiteOutput> {
    let mut s = Store::default().unwrap();
    let website = s.get_website(id).unwrap();
    Json(GetWebsiteOutput { 
        url: website.url 
    })
}


#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput> {
    let mut s = Store::default().unwrap();
    let id = s.sign_up(data.usernamer, data.password).unwrap();

    let response = CreateUserOutput {
        id:id
    };


    Json(response)
}

#[handler]
fn sign_in(json(data):Json<CreateUserInput>) -> Json<SigninOutput> {
    let mut s = Store::default().unwrap();

    let exists = s.sign_in(data.username, data.password).unwrap();


    let response = SigninOutput {
        jwt: String::from("vaibhav")
    };

    Json(response)
}




#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let mut s = Store::default().unwrap();
    let website = s.create_website(String::from("dd020379-1e62-44b2-8a3d-c4e17c30d044"), data.url).unwrap();
    let response = CreateWebsiteOutput{
        id:website.id
    };
    Jsron(response)
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app: Route = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("/user/signin", post(sign_in));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
