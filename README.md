# Headers Serializer
A lightweight crate specialized in serializing Http headers which is based on Rust's ```#[derive]``` mechanism. Usage is as simple as other built-in derive traits like Clone, Copy and Debug. 
It is also capable of serializing most rust structs as long as all the fields(or wrapped values) within have the ```to_string()``` methods.

# Usage
Add
``` 
    headers_serializer = { version = "0.1", git = "https://github.com/huxy11/headers_serializer.git"}
``` 
in Cargo.toml
and 
``` Rust
    #[macro_use] 
    extern crate headers_serializer; 
```    
at the root crate.

That's all we need to #[derive(ToMaps)] on our rust structs, or in most scenario, headers.

# Details
#[derive(ToMaps)] will automatically generate ```to_$LABEL()``` method onto the struct for each $LABEL defined above the strut's fields.

```to_$LABEL()``` method returns a HashMap contains all key-value paris labeled with corresponding name in which key is the filed name with all '_' replaced by '-'.

Option::Some(T) is serialized as T meanwhile Option::None will not be serialized and inserted into maps. 

# Example
Just simply derive and label fields like
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
The macro will generate corresponding functions according to the labels automatically.
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

