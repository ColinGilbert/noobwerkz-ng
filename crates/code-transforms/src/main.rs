use std::{fs, path::PathBuf};

fn main() {
    println!("Running code transform");

    let files_to_copy_out = vec![
        "components_common.rs".to_owned(),
        "entities_common.rs".to_owned(),
    ];

    let mut shared_base_path = PathBuf::new();
    shared_base_path.push("crates");
    shared_base_path.push("shared");
    shared_base_path.push("src");

    // First we check input files existence
    for f in files_to_copy_out.clone() {
        let mut shared_file_path = shared_base_path.clone();
        shared_file_path.push(f);
        let shared_file_string = shared_file_path
            .clone()
            .into_string()
            .expect("Could not stringify shared file path while doing the initial input file validating loop");
        if !fs::exists(shared_file_path.clone()).expect("Cannot check existence of file while doing the initial input file validating loop") {
            println!("Could not find input file to verify: {}", shared_file_string);
            return;
        }
    }

    // Now we validate client source code path
    let mut client_base_path = PathBuf::new();
    client_base_path.push("crates");
    client_base_path.push("client");
    client_base_path.push("src");

    if !fs::exists(client_base_path.clone()).expect("Could not check existence of client code path") {
        let client_path_string = client_base_path.into_string().expect("Could not stringify client base path");
        println!("Could not find client code path at {}", client_path_string);
        return;
    }

    // Now we validate server source code path
    let mut server_base_path = PathBuf::new();
    server_base_path.push("crates");
    server_base_path.push("server");
    server_base_path.push("src");

    if !fs::exists(server_base_path.clone()).expect("Could not check existence of server code path") {
        let server_path_string = server_base_path.into_string().expect("Could not stringify sever base path");
        println!("Could not find server code path at {}", server_path_string);
        return;
    }

    // Now we do the actual copying.
    for f in files_to_copy_out.clone() {
        let mut shared_file_path = shared_base_path.clone();
        shared_file_path.push(f);
        let shared_file_string = shared_file_path
            .clone()
            .into_string()
            .expect("Could not stringify shared file path");
        if !fs::exists(shared_file_path.clone()).expect("Cannot check existence of file while doing the code copying loop") {
            println!("Could not find input file to copy: {}", shared_file_string);
            return;
        }

        let mut to_client = client_base_path.clone();
        let mut to_server = server_base_path.clone();
        let file_name = shared_file_path.file_name();
        if file_name.is_some() {
        to_client.push(file_name.unwrap());
        to_server.push(file_name.unwrap());
        fs::copy(shared_file_path.clone(), to_client).unwrap();
        fs::copy(shared_file_path.clone(), to_server).unwrap();
        }
    }
}
