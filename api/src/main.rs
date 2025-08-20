use poem::{
    Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Json, Path},
};

use crate::{request_input::CreateWebsiteInput, request_output::CreateWebsiteOutput};
use store::Store;
pub mod request_input;
pub mod request_output;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    let store = Store {};
    store.create_user();
    format!("website: {}", website_id)
}

#[handler]
fn create_website(Json(_data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    // let url = data.url;
    // persisiting the data to a database
    // sqlx => pg library or diesel => prisma if we add here whats the point of having a monorepo
    let response = CreateWebsiteOutput {
        id: String::from("ID"),
    };
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3001"))
        .run(app)
        .await
}
