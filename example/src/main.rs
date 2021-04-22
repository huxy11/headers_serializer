#[macro_use]
extern crate serialize_to_maps;

#[allow(dead_code)]

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

#[derive(Default)]
struct WithToStringMethod {
    a: std::marker::PhantomData<i32>,
}
impl ToString for WithToStringMethod {
    fn to_string(&self) -> String {
        "石墨文档".to_string()
    }
}

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
