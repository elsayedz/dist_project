use elections::election_client::ElectionClient;
use elections::InitElectionRequest;
use elections::Empty;

use rand::Rng;
use rand::SeedableRng;
use rand::rngs::OsRng;

use std::env;
use std::thread::sleep;
use std::time::Duration;

pub mod elections {
    tonic::include_proto!("elections");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    
    let ip1 = format!("{}{}","http://" ,&args[1]);
    let ip2 = format!("{}{}","http://" ,&args[2]);
    let ip3 = format!("{}{}","http://" ,&args[3]);
    
    
    
    
    let client_1 = ElectionClient::connect(ip1).await?;
    let client_2 = ElectionClient::connect(ip2).await?;
    let client_3 = ElectionClient::connect(ip3).await?;
    // let mut fallenServer = ElectionClient::connect("http://10.40.32.92:50054").await?;
    
    let mut servers_connections = vec![client_1.clone(),client_2.clone(),client_3.clone()];
    let mut servers_connections_2 = vec![client_1.clone(),client_2.clone(),client_3.clone()];
    


    // let mut request = tonic::Request::new(
    //     InitElectionRequest{
    //         id: "2".to_owned(),   //to_owned() --> take a copy from the data
    //     }
    // );
    // let response = client.init_election(request).await;
    // println!("Init Election={:?}", response);


    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(6));
        loop {
            interval.tick().await;
            let mut rng = ::rand::rngs::StdRng::from_seed(OsRng.gen());
            let random_index = rng.gen_range(0..=servers_connections.len());
            println!("Thread spawned");
            let mut request = tonic::Request::new(
                InitElectionRequest{
                    id: "".to_owned(),  
                }
            );
            request.set_timeout(Duration::from_secs(1));
            let response = servers_connections[random_index].init_election(request).await;
            println!("Sent request to server {}", random_index);
            println!("Init election response={:?}", response);
        }
    });
    let mut i =0;
   loop {
        let mut request = tonic::Request::new(
            Empty{
               message: String::from("Test"),
            }
        );
        request.set_timeout(Duration::from_millis(1000));
        let res = servers_connections_2[i].ping(request).await
        .unwrap_or(tonic::Response::new(Empty{message: String::from("Server is down")}));
        
        println!("Ping Response from server {} ={:?}", i ,res);
        i = (i+1)%3;
        sleep(Duration::from_millis(2000));
        
        
        // let response_1 = client_1.ping(request).await;
        // println!("Ping Response={:?}", response_1);
        
        // let response_2 = client_2.ping(request).await;
        // println!("Ping Response={:?}", response_1);
        
        // let response_2 = client_3.ping(request).await;
        // println!("Ping Response={:?}", response_1);
    }
    
}