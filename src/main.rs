extern crate yaml_rust;
use std::fs;
use yaml_rust::YamlLoader;

fn load_yaml(path: &str) -> Vec<yaml_rust::Yaml> {
    let f = fs::read_to_string(path);
    let s = f.unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    docs
}

fn main() {
    let path = "./sample.yml";
    let docs = load_yaml(path);
    let doc = &docs[0];
}
