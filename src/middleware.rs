use elections::election_client::ElectionClient;
use elections::InitElectionRequest;

pub mod elections {
    tonic::include_proto!("elections");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut client = ElectionClient::connect("http://10.40.37.84:50051").await?;

    let request = tonic::Request::new(
        InitElectionRequest{
            id: "1".to_owned(),   //to_owned() --> take a copy from the data
        }
    );

    let response = client.init_election(request).await?;
    println!("RESPONSE={:?}", response);
    Ok(())
}