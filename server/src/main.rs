use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
struct ConfigFile {
    server: ServerConfig,
    s3: S3Config,
}

#[derive(Deserialize)]
struct ServerConfig {
    port: u32,
}

#[derive(Deserialize)]
struct S3Config {
    bucket: String,
    region: String,
    endpoint: String,
}

fn read_config_file() -> String {
    return fs::read_to_string("config.toml").expect("Something went wrong reading the file");
}

fn parse_config_file(configFileStr: String) -> ConfigFile {
    // return port, s3.bucket, s3.region, s3.endpoint
    return toml::from_str(&configFileStr).expect("Cannot parse file");
}

#[tokio::main]
async fn main() {
    // read config file
    let configFileStr = read_config_file();
    let configFile = parse_config_file(configFileStr);

    // connect to object storage
    
    // get list of all objects
    // make endpoint that get the list of all object
    // search with somes char and get list of all object which contain this somes chars
    // make endpoint to expose this function
    // download file
    // make endpoint to download file
    // upload file
    // make endpoint to upload file
    let awsConfig = Client::new({Region: "d"});


    println!("{}", configFile.server.port);
}
