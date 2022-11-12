use elections::election_client::ElectionClient;
use elections::InitElectionRequest;
use elections::Empty;

use std::time::Duration;
use std::thread::sleep;

pub mod elections {
    tonic::include_proto!("elections");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let mut client = ElectionClient::connect("http://192.168.1.12:50051").await?;
    let mut fallenServer = ElectionClient::connect("http://192.168.1.12:50053").await?;
    
    
    let request = tonic::Request::new(
        Empty{
           message: String::from("Test"),   //to_owned() --> take a copy from the data
        }
    );
    let response = client.ping(request).await;
    println!("Ping Response={:?}", response);


    let mut request = tonic::Request::new(
        InitElectionRequest{
            id: "2".to_owned(),   //to_owned() --> take a copy from the data
        }
    );
    let response = client.init_election(request).await;
    println!("RESPONSE={:?}", response);

   loop {
        println!("Loop started");
        // let ten_sec = time::Duration::from_millis(2000);
        sleep(Duration::from_millis(10000));
        println!("I'm alive");
        let request = tonic::Request::new(
            InitElectionRequest{
                id: "2".to_owned(),   //to_owned() --> take a copy from the data
            }
        );
        let response = client.init_election(request).await;
        println!("RESPONSE={:?}", response);

        let request = tonic::Request::new(
            Empty{
               message: String::from("Test"),   //to_owned() --> take a copy from the data
            }
        );
        let response = fallenServer.ping(request).await;
        println!("RESPONSE From Fallen server={:?}", response);
    }
    
}