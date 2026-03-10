// Rust版cJSON核心数据结构
#[derive(Debug)]
enum JsonValue {
    String(String),
    Number(f64),
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
}
/// 参数：待解析的JSON字符串（如 "\"Tom\"" / "18.0"）
/// 返回值：解析后的JsonValue（None表示解析失败）
fn parse_json(s: &str) -> Option<JsonValue> {
    // 第一步：跳过字符串前后的空白字符
    let s_trimmed = s.trim();
    if s_trimmed.is_empty() {
        return None; // 空字符串解析失败
    }

    // 第二步：解析字符串类型（以"开头且以"结尾）
    if s_trimmed.starts_with('"') && s_trimmed.ends_with('"') {
        // 提取引号中间的内容（去掉首尾的"）
        let content = &s_trimmed[1..s_trimmed.len() - 1];
        return Some(JsonValue::String(content.to_string()));
    }

    // 第三步：解析数字类型（浮点数，兼容整数/小数）
    if let Ok(num) = s_trimmed.parse::<f64>() {
        return Some(JsonValue::Number(num));
    }

    None
}
//解析函数结束
///JSON打印函数：把JsonValue转为JSON字符串
fn print_json(value: &JsonValue) -> String {
    match value {
        // 字符串类型：加双引号（符合JSON格式）
        JsonValue::String(s) => format!("\"{}\"", s),
        // 数字类型：直接转字符串
        JsonValue::Number(n) => n.to_string(),
        // 对象类型(只输出键值对，不用缩进）
        JsonValue::Object(obj) => {
            let mut res = "{".to_string();
            for (k, v) in obj {
                res.push_str(&format!("{}: {}, ", k, print_json(v)));
            }
            // 去掉最后一个多余的逗号
            if res.len() > 1 {
                res.pop();
                res.pop();
            }
            res.push('}');
            res
        },
        //只输出值，不用缩进
        JsonValue::Array(arr) => {
            let mut res = "[".to_string();
            for v in arr {
                res.push_str(&format!("{}, ", print_json(v)));
            }
            // 去掉最后一个多余的逗号
            if res.len() > 1 {
                res.pop();
                res.pop();
            }
            res.push(']');
            res
        },
    }
}
//打印函数结束
fn main() {
    // 1. 测试：创建简单JSON数据结构
    let json = JsonValue::Object(vec![
        ("name".to_string(), JsonValue::String("Tom".to_string())),
        ("age".to_string(), JsonValue::Number(18.0)),
    ]);

    // 2. 测试：打印JSON字符串
    println!("=== Rust版JSON打印结果 ===");
    println!("{}", print_json(&json));

    // 3. 测试：简单解析
    println!("\n=== Rust版JSON解析测试 ===");
    println!("解析字符串：{:?}", parse_json("\"Tom\""));
    println!("解析数字：{:?}", parse_json("18"));
}
