
pub struct Config {
    name: String
    //Need to periodic update path for config
    path: String
}


/// Load config data
impl Config {
    pub fn(path: String) -> Config {
        Config{name: "Default"}
    }
}
