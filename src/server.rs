use tonic::{transport::Server, Request, Response, Status};

use elections::election_server::{Election, ElectionServer};
use elections::election_client::ElectionClient;
use elections::{Empty, InitElectionRequest, BroadcastId};

use std::collections::HashMap;


pub mod elections {
    tonic::include_proto!("elections");
}


#[derive(Debug, Default)]
pub struct MyElection {
    ip_to_id : HashMap<String,String>
}

#[tonic::async_trait]
impl Election for MyElection {

    async fn init_election(
        &self,
        request: Request<InitElectionRequest>,
    ) -> Result<Response<Empty>, Status> {
        println!("Got a init_election request: {:?}", request);

        let reply = elections::Empty {
            message: format!("ACK!").into(),
        };

        // let mut client = ElectionClient::connect("http://10.40.37.84:9999").await.unwrap();
        // let request = tonic::Request::new(
        //     Empty {
        //         message: format!("Time to go down").into(),
        //     }
        // );
    
        // let response = client.force_failure(request).await?;
    
        // println!("RESPONSE={:?}", response);

        Ok(Response::new(reply))
    }

    async fn broadcast_id(
        &self,
        request: Request<BroadcastId>,
    ) -> Result<Response<Empty>, Status> {
        println!("Got broadcast_id (new_id) request: {:?}", request);

        

        let reply = elections::Empty {
            message: format!("").into(),
        };

        let ip = request.remote_addr().unwrap().ip().to_string();
        println!("IP: {}", ip);
        // ip_to_id[ip] = request.request.id;

        Ok(Response::new(reply))
    }

    async fn force_failure(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Empty>, Status> {
        println!("Got force_failure request: {:?} \n This Server will GO DOWN!!", request);

        let reply = elections::Empty {
            message: format!("").into(),
        };

        Ok(Response::new(reply))
    }
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "10.40.39.4:50051".parse()?;
    let election_service = MyElection::default();

    Server::builder()
        .add_service(ElectionServer::new(election_service))
        .serve(addr)
        .await?;

    Ok(())
}