// Rust版cJSON核心数据结构
// 枚举：表示JSON的所有基础类型
#[derive(Debug)] // 加这个注解，才能打印数据结构（方便测试）
enum JsonValue {
    String(String),    // JSON字符串类型
    Number(f64),       // JSON数字类型
    Object(Vec<(String, JsonValue)>), // JSON对象
    Array(Vec<JsonValue>),            // JSON数组
}

fn main() {
    // 测试：创建一个简单的JSON对象（对应C版{"name":"Tom","age":18,"grade":90.5}）
    let json_data = JsonValue::Object(vec![
        // 键值对1：name → "Tom"
        ("name".to_string(), JsonValue::String("Tom".to_string())),
        // 键值对2：age → 18（自动转为浮点数）
        ("age".to_string(), JsonValue::Number(18.0)),
        // 键值对3：grade → 90.5
        ("grade".to_string(), JsonValue::Number(90.5)),
        // 额外加一个数组：hobby → ["read", "run"]
        ("hobby".to_string(), JsonValue::Array(vec![
            JsonValue::String("read".to_string()),
            JsonValue::String("run".to_string()),
        ])),
    ]);

    // 打印创建的JSON数据结构（验证是否成功）
    println!("=== Rust版cJSON数据结构 ===");
    println!("{:#?}", json_data); // {:#?} 是格式化打印，看起来更清晰
}
