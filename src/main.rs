use axum::Router;
use axum::routing::get;

pub mod clients_info {
    use mongodb::{Client, Collection};
    use mongodb::bson::{doc, Document};

    pub fn uri_from_client() -> String {
        String::from("mongodb://localhost:27017")
    }

    pub fn lower_uri() -> String {
        uri_from_client().to_lowercase()
    }

    pub async fn on_ready(data: String) ->mongodb::error::Result<()> {
        let uri = lower_uri();

        let client = Client::with_uri_str(uri).await?;

        let database = client.database("app");
        let coll: Collection<Document> = database.collection("backend");

        coll.insert_one(doc! {"data": data}).await.expect("TODO: panic message");

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


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(myinit::send_message));
    let listener = tokio::net::TcpListener::bind
        ("0.0.0.0:5000").await.unwrap();

    clients_info::on_ready(myinit::send_message().await).await.expect("TODO: panic message");

    axum::serve(listener, app).await.unwrap();
}