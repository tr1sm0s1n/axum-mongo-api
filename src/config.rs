use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

pub async fn connect() -> mongodb::error::Result<Client> {
    // Replace the placeholder with your Atlas connection string
    let uri = "mongodb://root:rootpw@127.0.0.1:27017/axum-mongo?authSource=admin";
    let mut client_options = ClientOptions::parse(uri).await?;

    // Set the server_api field of the client_options object to Stable API version 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Create a new client and connect to the server
    let client = Client::with_options(client_options)?;

    // Send a ping to confirm a successful connection
    client
        .database("axum-mongo")
        .run_command(doc! { "ping": 1 })
        .await?;
    println!("Pinged your database. Successfully connected to MongoDB!");

    Ok(client)
}
