
pub struct Config {
    name: String,
    //Need to periodic update path for config
    path: String
}


/// Load config data
impl Config {
    fn new(path: String) -> Config {
        Config{name: path, path:path}
    }
}
