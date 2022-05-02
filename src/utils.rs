use reqwest::Client;

/*
concat! takes any number of comma-separated literals and gives
env! expands to the value of the named environment variable at compile time
*/ 
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn get_client() -> Client {
    let client = Client::builder()         //builder() is used to handle the failure as an Error instead of panicking
        .user_agent(APP_USER_AGENT)        //Setting the User-Agent header to be used by this client
        .build()                           //returns a client
        .unwrap();                         //returns the result if gets something and panics if gets none

    client
}
