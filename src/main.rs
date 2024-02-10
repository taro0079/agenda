extern crate yaml_rust;
use std::fs;
use yaml_rust::{YamlEmitter, YamlLoader};

fn load_yaml(path: &str) -> Vec<yaml_rust::Yaml> {
    let f = fs::read_to_string(path);
    let s = f.unwrap().to_string();
    let docs = YamlLoader::load_from_str(&s).unwrap();
    docs
}

type Paths = Vec<Path>;
struct Path {
    path_name: String,
    endpoints: Vec<Endpoint>,
}

struct Endpoint {
    method: Method,
    request_body: RequestBody,
}

enum Method {
    Get,
    Post,
    Put,
    Delete,
}

struct RequestBody {
    description: String,
    content: Content,
}

struct Content {
    schema_type: SchemaType,
}

struct SchemaType {
    schema: Schema,
}

struct Schema {
    ty: String,
    properties: Properties,
}

struct Properties {
    properties: Vec<Property>,
}
struct Property {
    name: String,
    ty: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "./sample.yml";
    let docs = load_yaml(path);
    let doc = &docs[0];
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&doc["openapi"]).unwrap();
    }
    println!("{}", out_str);
    Ok(())
}
