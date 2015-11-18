
pub struct Config {
    name: String,
    //Need to periodic update path for config
    path: String
}


/// Load config data
impl Config {
    fn new(name: String, path: String) -> Config {
        Config{name: name, path:path}
    }
}
