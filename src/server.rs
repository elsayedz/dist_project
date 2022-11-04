use tonic::{transport::Server, Request, Response, Status};

use elections::election_server::{Election, ElectionServer};
use elections::election_client::ElectionClient;
use elections::{Empty, InitElectionRequest, BroadcastId};

use std::collections::HashMap;
use std::env;
use http::Uri;


pub mod elections {
    tonic::include_proto!("elections");
}


#[derive(Debug, Default)]
pub struct MyElection {
    pub id_to_ip : HashMap<String, String>
}

#[tonic::async_trait]
impl Election for MyElection {

    async fn init_election(
        &self,
        request: Request<InitElectionRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = request.into_inner().id;
        println!("Got a init_election request: {:?}", &id);

        let reply = elections::Empty {
            message: format!("ACK!").into(),
        };

        let mut client = ElectionClient::connect(self.id_to_ip.get(&id).unwrap().parse::<Uri>().unwrap()).await.unwrap();
        let request = tonic::Request::new(
            Empty {
                message: format!("Time to go down").into(),
            }
        );
    
        let response = client.force_failure(request).await?;
    
        println!("RESPONSE={:?}", response);

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

impl MyElection {
    pub fn new(_ip1:String, _ip2:String, _ip3:String) -> Self {
        // let mut vec = Vec::new();
        // vec.push(_ip1);
        // vec.push(_ip2);
        // vec.push(_ip3);

        let mut ipID: HashMap<String, String> = HashMap::new();
        ipID.insert("1".to_string(), _ip1);
        ipID.insert("2".to_string(), _ip2);
        ipID.insert("3".to_string(), _ip3);

        Self {
            id_to_ip: ipID,
        }
    }
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    let ip = &args[1];

    let addr = ip.parse()?;

    let election_service = MyElection::new(ip.to_string(), String::from("http://10.40.39.4:50053"),
    String::from("http://10.40.39.4:50054"));

    Server::builder()
        .add_service(ElectionServer::new(election_service))
        .serve(addr).await?;

    Ok(())
}