use std::fmt;
use axum::Router;
use axum::routing::get;
use crate::types::Types;

pub mod types {
    pub struct Types {
        text: String
    }

    impl Types {
        pub fn get_(&self) -> String {
            self.text.to_string()
        }

        pub(crate) fn new(data: String) -> Types {
            Types {
                text: data.to_string(),
            }
        }
    }
}

pub mod clients_info {
    use mongodb::{Client, Collection};
    use mongodb::bson::{doc, Document};

    pub fn uri_from_client() -> String {
        String::from("mongodb://localhost:27017")
    }

    pub fn lower_uri() -> String {
        uri_from_client().to_lowercase()
    }


    pub async fn on_ready(data: String) -> mongodb::error::Result<()> {
        let uri = lower_uri();

        if !uri.trim().is_empty() {
            let client = Client::with_uri_str(uri).await?;

            let database = client.database("app");
            let coll: Collection<Document> = database.collection("backend");

            coll.insert_one(doc! {"data": data}).await.expect("TODO: panic message");
        }

        Ok(())
    }
}

pub mod myinit {
    pub fn get_lower_message(str: &str) -> String {
        str.to_lowercase()
    }

    pub async fn send_message() -> String {
        get_lower_message("Hello I am Learning Rust Programming")
    }
}

#[derive(Debug)]
enum Route {
    Home,
    About,
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Route::Home => {
                write!(f, "/")
            }

            Route::About => {
                write!(f, "/About")
            }
        }
    }
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    const IS_WORKING: bool = false;

    if !IS_WORKING.to_string().is_empty() {
        let app = Router::new().route(&*Route::Home.to_string(), get(myinit::send_message))
            .route(&*Route::About.to_string(), get(myinit::get_lower_message("Hello And Welcome to About Page")));
        let listener = tokio::net::TcpListener::bind
            ("0.0.0.0:5000").await.unwrap();


        let t: Types = Types::new(myinit::send_message().await);

        clients_info::on_ready(t.get_()).await.expect("TODO: panic message");

        axum::serve(listener, app).await.unwrap();
    }
}