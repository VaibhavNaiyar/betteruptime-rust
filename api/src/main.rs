use poem::{Route, Server, get, handler, http::response, listener::TcpListener, post, web::{Json, Path}};

use crate::{request_inputs::CreateWebsiteInput, request_outputs::CreateWebsiteOutput};


pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("website: {}", website_id)
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let url = data.url;
    
    let response = CreateWebsiteOutput {
        id:data.url
    };
    //persist this in DB
    Json(response)

}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app: Route = Route::new()
        .at("/status/:website_id", get(get_website).post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
