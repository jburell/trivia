const HOSTNAME: &'static str = "127.0.0.1";
const PORT: &'static str = "8080";

// TODO: pick up from config file
pub fn get_base_url() -> String {
    format!("{}:{}", HOSTNAME, PORT).to_owned()
}