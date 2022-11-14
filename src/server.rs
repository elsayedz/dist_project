use tonic::transport::Channel;
use tonic::{transport::Server, Request, Response, Status};

use elections::election_server::{Election, ElectionServer};
use elections::election_client::ElectionClient;
use elections::{Empty, InitElectionRequest, BroadcastId};

use futures::lock::Mutex;
use std::time::Duration;
use std::thread::sleep;

use std::collections::HashMap;
use std::env;
use std::net::{SocketAddr};
use std::sync::{Arc};
use http::Uri;


pub mod elections {
    tonic::include_proto!("elections");
}



#[derive(Debug, Default, Clone)]
pub struct MyElection {
    // pub my_server: MyServer
    // value : Arc<Mutex<bool>>,
    // pub id_to_ip : HashMap<String, String>,
    pub id_to_ip : Arc<Mutex<HashMap<String, String>>>,
    pub is_working : Arc<Mutex<bool>>,
    pub my_id : Arc<Mutex<String>>,
    pub connections : Arc<Mutex<Vec<ElectionClient<Channel>>>>
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
        let mut map = self.id_to_ip.lock().await;

        for (key, _value) in &*map {
            if key.parse::<i32>().unwrap() > min_id {
                min_id = key.parse::<i32>().unwrap();
            }
        }
        println!("Min id: {:?}", min_id);
        match map.get(&min_id.to_string()){
            Some(ip) => {
                println!("Min id: {:?}", min_id);
                println!("Min id ip: {:?}", ip);
                // let mut client = ElectionClient::connect(ip.parse::<Uri>().unwrap()).await;
                let mut client = ElectionClient::connect(ip.parse::<Uri>().unwrap()).await.unwrap();
                let mut request = tonic::Request::new(
                    Empty {
                        message: format!("Time to go down").into(),
                    }
                );
                
                request.set_timeout(Duration::from_millis(1000));
                let response = client.force_failure(request).await?;
                println!("Sent request to server {}", min_id);
                println!("Broadcast id response={:?}", response);
            },
            None => {
                println!("Couldn't find min id in Map");
            }
        }
        // let mut client = ElectionClient::connect(map.get(&min_id.to_string()).unwrap().parse::<Uri>().unwrap()).await.unwrap();
        
    
        // println!("Force Failure Response ={:?}", response.get_ref().message);

        Ok(Response::new(reply))
    }

    async fn broadcast_id(&self,request: Request<BroadcastId>,) -> Result<Response<Empty>, Status> {
        println!("-----Got a broadcast_id (new_id) request------\n");
        println!("Old id: {:?}", request.get_ref().old_id);
        println!("New id: {:?}", request.get_ref().new_id);
        println!("New ip: {:?}", request.get_ref().new_ip);

        let mut ip_map = self.id_to_ip.lock().await;
        ip_map.remove(&request.get_ref().old_id);
        ip_map.insert(request.get_ref().new_id.clone(), request.get_ref().new_ip.clone());


        let reply = elections::Empty {
            message: format!("Updated IP map Successfully").into(),
        };
        
        Ok(Response::new(reply))
    }

    async fn force_failure(&self, request: Request<Empty>) -> Result<Response<Empty>, Status> {
        println!("Got force_failure request from: {:?} \n This Server will GO DOWN!!", request.remote_addr());

        sleep(Duration::from_millis(30000));
        println!("Server is UP again");

        let mut ip_map = self.id_to_ip.lock().await;
        let mut my_id = self.my_id.lock().await;
        
        let my_ip = ip_map.get(&*my_id).unwrap().clone();   // My cuurent ip
        let max_id = ip_map.keys().max().unwrap().clone();      // Max id in the network
        ip_map.remove(&*my_id);                 // Remove my id from the map
        
        let id_as_int = max_id.parse::<i32>().unwrap();        // Convert max id to int
        let new_id = id_as_int + 1;     // Increment max id by 1
        println!("Removed myself from the map");


        ip_map.insert(new_id.to_string(), my_ip.clone());
        
        let mut connection_vec = self.connections.lock().await;
        
        for i in 0..connection_vec.len() {
            let mut request = tonic::Request::new(
                BroadcastId {
                    old_id: my_id.to_owned().clone(),
                    new_id: new_id.to_string().clone(),
                    new_ip: my_ip.clone(),
                }
            );
            request.set_timeout(Duration::from_millis(1000));
            let response = connection_vec[i].broadcast_id(request).await;
            println!("Sent request to server {}", new_id);
            println!("Broadcast id response={:?}", response);
        }
        
        let mut update_id = self.my_id.lock().await;
        *update_id = new_id.to_string();
        // let mut mut_value = self.is_working.lock().unwrap();
        // *mut_value = false;
        
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
    pub async fn new(_ip1:String, _ip2:String, _ip3:String, _my_id:String) -> Self {
        // let mut vec = Vec::new();
        // vec.push(_ip1);
        // vec.push(_ip2);
        // vec.push(_ip3);

        let mut ip_id: HashMap<String, String> = HashMap::new();
        ip_id.insert("0".to_string(), _ip1.clone());
        ip_id.insert("1".to_string(), _ip2.clone());
        ip_id.insert("2".to_string(), _ip3.clone());

        let mut servers_connections: Vec<ElectionClient<Channel>> = Vec::new();
        println!("*********************");
        for (key, value) in &ip_id {
            if key != &_my_id {
               servers_connections.push(ElectionClient::connect(value.parse::<Uri>().unwrap()).await.unwrap()); 
               println!("Server id: {} --> IP {}", key, value);
            }else{
                println!("Main Server id: {} --> IP {}", key, value);
            }
        }
        println!("*********************");
        
        Self {
                id_to_ip: Arc::new(Mutex::new(ip_id)),
                is_working: Arc::new(Mutex::new(true)),
                my_id: Arc::new(Mutex::new(_my_id)),
                connections: Arc::new(Mutex::new(servers_connections)),
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

    let election_service = MyElection::new(ip1, ip2, ip3, main_server_index.to_string()).await;
    Server::builder()
        .add_service(ElectionServer::new(election_service))
        .serve(server_addr).await?;

    Ok(())
}
