use tonic::{transport::Server, Request, Response, Status};

use elections::election_server::{Election, ElectionServer};
use elections::election_client::ElectionClient;
use elections::{Empty, InitElectionRequest, BroadcastId};

use std::time::Duration;
use std::thread::sleep;

use std::collections::HashMap;
use std::env;
use std::net::{SocketAddr};
use std::sync::{Arc, Mutex};
use http::Uri;


pub mod elections {
    tonic::include_proto!("elections");
}


#[derive(Debug, Default)]
pub struct MyElection {
    // pub my_server: MyServer
    // value : Arc<Mutex<bool>>,
    pub id_to_ip : HashMap<String, String>,
    pub is_working : Arc<Mutex<bool>>
}

#[derive(Debug, Default)]
pub struct MyServer {
    
}

#[tonic::async_trait]
impl Election for MyElection {
    
    async fn init_election(&self, request: Request<InitElectionRequest>) -> Result<Response<Empty>, Status> {
        let id = request.into_inner().id;
        println!("Got a init_election request: {:?}", &id);

        let reply = elections::Empty {
            message: format!("ACK!").into(),
        };

        let mut min_id = std::i32::MAX;
        for (key, _value) in &self.id_to_ip {
            if key.parse::<i32>().unwrap() < min_id {
                min_id = key.parse::<i32>().unwrap();
            }
        }
        println!("Min id: {:?}", min_id);
        let mut client = ElectionClient::connect(self.id_to_ip.get(&min_id.to_string()).unwrap().parse::<Uri>().unwrap()).await.unwrap();
        let mut request = tonic::Request::new(
            Empty {
                message: format!("Time to go down").into(),
            }
        );
        
        request.set_timeout(Duration::from_millis(1000));
        let response = client.force_failure(request).await?;
    
        println!("Force Failure Response ={:?}", response.get_ref().message);

        Ok(Response::new(reply))
    }

    async fn broadcast_id(&self,request: Request<BroadcastId>,) -> Result<Response<Empty>, Status> {
        println!("Got broadcast_id (new_id) request: {:?}", request);

        let reply = elections::Empty {
            message: format!("").into(),
        };

        let ip = request.remote_addr().unwrap().ip().to_string();
        println!("IP: {}", ip);
        // ip_to_id[ip] = request.request.id;

        Ok(Response::new(reply))
    }

    async fn force_failure(&self, request: Request<Empty>) -> Result<Response<Empty>, Status> {
        println!("Got force_failure request from: {:?} \n This Server will GO DOWN!!", request.remote_addr());

        sleep(Duration::from_millis(30000));
        println!("Server is UP again");

        let mut mut_value = self.is_working.lock().unwrap();
        *mut_value = false;
        
        let reply = elections::Empty {
            message: format!("").into(),
        };

        Ok(Response::new(reply))
    }

    async fn ping(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Empty>, Status> {
        // let value = self.is_working.lock().unwrap();
        // if *value != false {
        //     println!("Got ping request from: {:?}\n Message:{:?}", request.remote_addr() ,request.get_ref().message);
        //     let reply = elections::Empty {
        //         message: format!("").into(),
        //     };
        //     Ok(Response::new(reply))
        // } else {
        //     println!("Got ping request while server is down,");
        //     Err(Status::unavailable("Server is down"))
        // }
        println!("Got ping request from: {:?}\n Message:{:?}", request.remote_addr() ,request.get_ref().message);
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

        let mut ip_id: HashMap<String, String> = HashMap::new();
        ip_id.insert("1".to_string(), _ip1);
        ip_id.insert("2".to_string(), _ip2);
        ip_id.insert("3".to_string(), _ip3);

        Self {
                id_to_ip: ip_id,
                is_working: Arc::new(Mutex::new(true))
        }
    }
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    let ip_server1 = &args[1];
    let ip_server2 = &args[2];
    let ip_server3 = &args[3];
    
    let main_server_index = &args[4].parse::<usize>().unwrap();
    
    let servers = vec![ip_server1, ip_server2, ip_server3];

    let server_addr:SocketAddr = servers[*main_server_index].parse().unwrap();
    let ip1 = format!("{}{}","http://" ,server_addr);
    let ip2 = format!("{}{}","http://" ,ip_server2);
    let ip3 = format!("{}{}","http://" ,ip_server3);
    println!("Main server listening on: {}", servers[*main_server_index]);
    // println!("Server2 listening on {}", ip2);
    // println!("Server3 listening on {}", ip3);

    // let mut myServer = MyElection::new(ip.to_string(), String::from("http://10.40.32.92:50053"),
    // String::from("http://10.40.32.92:50054"));

    let election_service = MyElection::new(ip1, ip2, ip3);
    Server::builder()
        .add_service(ElectionServer::new(election_service))
        .serve(server_addr).await?;

    Ok(())
}