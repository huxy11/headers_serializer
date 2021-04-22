# Serialize to Maps
A handy tool specializing in serializing Rust data structures into HashMap(maybe more if necessary) which is based on Rust's #[derive] mechanism, just like what you would use to automatically derive implementations of the built-in Clone, Copy, Debug, or other traits. It is able to generate implementations for most structs as long as all the fields(or wrapped value, for options) have the to_string() methods.

# Usage
add
``` 
    serialize_to_maps = { version = "0.1", git = "https://github.com/huxy11/serialize_to_maps.git"}
``` 
in Cargo.toml
and then
``` Rust
    #[macro_use] 
    extern crate serialize_to_maps; 
```    
at the root crate.

# Example
Just simply derive like 
``` Rust
#[derive(SerializeToMaps)]
struct TestStruct<'a> {
    #[label("my_map")]
    num: f64,
    #[label("my_map")]
    string: String,
    #[label("my_map")]
    string_opt: Option<String>,
    #[label("my_map")]
    string_opt_sec: Option<String>, 
    #[label("my_map")]
    str_ref: &'a str,

    #[label("another_map")]
    structure: WithToStringMethod,
    #[label("another_map")]
    structure_opt: WithToStringMethod,

    unlabeled: i32,
}
``` 
The macro will generate corresponding functions automately.
``` Rust
fn main() {
    let test_struct = TestStruct {
        num: 2014.,
        string: "Shimo".to_string(),
        string_opt: Some("WenDang".to_string()),
        string_opt_sec: None, // None will not be serialize
        str_ref : "shi mo wen dang",
        structure: WithToStringMethod::default(),
        structure_opt: WithToStringMethod::default(),
        unlabeled: 56,
    };
    let map_1 = test_struct.to_my_map();
    let map_2 = test_struct.to_another_map();
    println!("map1 => {:?}", map_1);
    println!("map2 => {:?}", map_2);
}
```
Out put
```
map1 => {"num": "2014", "string-opt": "WenDang", "str-ref": "shi mo wen dang", "string": "Shimo"}
map2 => {"structure-opt": "石墨文档", "structure": "石墨文档"}
```
