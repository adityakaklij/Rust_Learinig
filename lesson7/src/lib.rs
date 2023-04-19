#![allow(dead_codem unused_variables)]
// Creating a authentication module

pub struct Credentials {
    username: String,
    password: String,
}

enum Status {
    Connected, 
    Interrupted,
}

fn connected_to_db() -> Status {
    return Status::Connected;
}


fn logout() {
    
}

fn get_user() {
    
}

fn login() {
    get_user();
}

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = connected_to_db() {
        login(creds);
    }
}


