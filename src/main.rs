mod sfu;

use tonic::{Request, Response, Status, transport::Server};

use bookstore::{GetBookRequest, GetBookResponse};
use bookstore::bookstore_server::{Bookstore, BookstoreServer};

mod bookstore {
    include!("bookstore.rs");
}

#[derive(Default)]
pub struct BookStoreImpl {}

#[tonic::async_trait]
impl Bookstore for BookStoreImpl {
    async fn get_book(&self, request: Request<GetBookRequest>) -> Result<Response<GetBookResponse>, Status> {
        println!("Request from {:?}", request.remote_addr());

        let response = GetBookResponse {
            id: request.into_inner().id,
            author: "Peter".to_owned(),
            name: "Zero to One".to_owned(),
            year: 2014,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:5050".parse().unwrap();
    let bookstore = BookStoreImpl::default();

    println!("Bookstore server listening on {}", addr);

    let service = tonic_web::config()
        .allow_origins(vec!["http://localhost:3000"])
        .enable(BookstoreServer::new(bookstore));

    Server::builder()
        .accept_http1(true)
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}