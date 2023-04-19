
// Creating the modules for each common function to make the code clea
// As for auth we can differentate the modules like, 
// 1. DB,  2. auth, 3. User

mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connected_to_db() -> Status {
        return Status::Connected;
    }

    pub fn get_user() {

    }
}

mod auth_utils {
    pub fn login(creds: models:: Credentials){ // relative path for the Credentials
        crate::database:: get_user(); // Absolute path
    }

    pub fn logout() {

    }
}

mod models {
    pub struct Credentials {
        username: String,
        password: String,
    }
}

pub fn authenticatie(creds:auth_utils::models:: Credentials){
    if let database:: Status:: Connected = connected_to_db() {
        auth_utils::login(creds);
    }
}