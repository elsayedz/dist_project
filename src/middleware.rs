use elections::election_client::ElectionClient;
use elections::InitElectionRequest;

use std::time::Duration;
use std::thread::sleep;

pub mod elections {
    tonic::include_proto!("elections");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let mut client = ElectionClient::connect("http://10.40.39.4:50051").await?;
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
        sleep(Duration::from_millis(2000));
        println!("I'm alive");
        let request = tonic::Request::new(
            InitElectionRequest{
                id: "2".to_owned(),   //to_owned() --> take a copy from the data
            }
        );
        let response = client.init_election(request).await;
        println!("RESPONSE={:?}", response);

    }
    
}