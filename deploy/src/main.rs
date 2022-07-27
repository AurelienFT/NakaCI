use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct SshInfos {
    ip: String,
    user: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum ServerConnection {
    #[serde(rename = "ssh")]
    Ssh(SshInfos)
}

#[derive(Debug, Serialize, Deserialize)]
struct File {
    connections: HashMap<String, ServerConnection>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("./examples/basic.yaml")?;
    let d: File = serde_yaml::from_reader(f)?;
    println!("Read YAML string: {:#?}", d);
    Ok(())
}
