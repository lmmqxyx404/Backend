use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    name: String,
    value: i32,
}

// Handler that generates and serves YAML
/// 生成 yaml 接口
pub async fn serve_yaml() -> String {
    // Define a struct to serialize

    // Create an instance of the struct
    let my_struct = MyStruct {
        name: "Examples".to_string(),
        value: 42,
    };

    // Serialize the struct to a YAML string
    let yaml_string = serde_yaml::to_string(&my_struct).unwrap();

    // Serve the YAML string
    yaml_string
}

