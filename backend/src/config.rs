use serde_yaml::{from_reader, Value};
use std::collections::HashMap;
use std::env;

/// Holds configuration parameters for the application
///
/// # Attributes
/// * map (HashMap<String, Value>): the map of parameters for the app
pub struct Config {
    pub map: HashMap<String, Value>,
}

impl Config {
    /// Creates a new Config map with loaded data from a yml file where the path is from the last argument
    /// passed into the program.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (Config): a loaded Config struct
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[args.len() - 1];

        let file = std::fs::File::open(file_path).unwrap();
        Config {
            map: from_reader(file).unwrap(),
        }
    }
}
