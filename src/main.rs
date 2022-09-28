use tonic::{
    transport::Server,
    Request,
    Response,
    Status
};

use sn::{
    AccessRequest,
    AccessResponse,
    AddAccessRequest,
    AddAccessResponse,
    sn_server::{
        Sn,
        SnServer
    }
};

pub mod sn {
    tonic::include_proto!("sn");
}

#[derive(Debug, Default)]
pub struct SNService {}
#[tonic::async_trait]
impl Sn for SNService {
    async fn has_access(&self, request: Request<AccessRequest>) -> Result<Response<AccessResponse>, Status> {
        let sn_request = request.into_inner();
        let (user, information_request) = (sn_request.user, sn_request.information_request);
        println!("{}: {}", &user, &information_request);
        Ok(Response::new(AccessResponse{ has_access: false }))
    }

    async fn add_access(&self, request: Request<AddAccessRequest>) -> Result<Response<AddAccessResponse>, Status> {
        let sn_request = request.into_inner();
        let (user, information_request) = (sn_request.user, sn_request.information_request);
        println!("{}: {}", &user, &information_request);
        Ok(Response::new(AddAccessResponse{ approved: false }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:50051".parse().unwrap();
    // creating a service
    let silentninja_service = SNService::default();
    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(SnServer::new(silentninja_service))
        .serve(addr)
        .await?;
    Ok(())
}

// grpcurl -plaintext -import-path ./proto -proto sn_server.proto -d '{"user": "you", "information_request": "nope"}' '[::]:50051' sn.SN/HasAccess

