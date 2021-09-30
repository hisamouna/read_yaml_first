mod utils;

use utils::file::read_yaml;
use serde_yaml::Value;

fn main() {
    let result = read_yaml(String::from("tmp/test.yaml"));
    match result {
        Ok(x) => {
            println!("{:?}",x);
            let a : Value = Value::String("a".to_string());
            println!("{:?}",x.get(&a).unwrap())
        }
        Err(e) => println!("Error: {}",e),
    }
}
