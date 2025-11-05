use crate::opts::OutputFormat;
use csv::Reader;
use serde_json::Value as SerdeValue;
use std::collections::BTreeMap;
use std::process::id;
use toml::Value as TomlValue;
use toml::map::Map;

pub fn process_csv(
    input_file: &str,
    output_file: &str,
    format: OutputFormat,
) -> anyhow::Result<()> {
    println!("Processing {}, {}, {:?}", input_file, output_file, format);

    let mut reader = Reader::from_path(input_file)?;
    let header = reader.headers()?.clone();

    let content = match format {
        OutputFormat::Json | OutputFormat::Yaml => {
            let mut record_list = vec![];

            for record in reader.records() {
                let rec = record?;
                let zipped = header.iter().zip(rec.iter()).collect::<SerdeValue>();
                record_list.push(zipped);
            }

            match format {
                OutputFormat::Json => serde_json::to_string_pretty(&record_list)?,
                OutputFormat::Yaml => serde_yaml::to_string(&record_list)?,
                _ => "".to_string(),
            }
        }

        OutputFormat::Toml => {

            let mut record_list = BTreeMap::new();

            for (idx, record) in reader.records().enumerate() {
                let rec = record?;
                let map = header
                    .iter()
                    .zip(rec.iter())
                    .map(|(key, value)| (key.to_string(), TomlValue::String(value.to_string())))
                    .collect::<Map<String, TomlValue>>();

                record_list.insert(idx.to_string(), TomlValue::Table(map));
            }

            // let mut root = BTreeMap::new();
            // root.insert("items".to_string(), TomlValue::Array(record_list));

            toml::to_string(&record_list)?
        }
    };

    // let mut record_list = vec![];
    //
    // for record in reader.records() {
    //     let rec = record?;
    //     let zipped = header.iter().zip(rec.iter()).collect::<Value>();
    //     record_list.push(zipped);
    // }
    //
    // let content = match format {
    //     OutputFormat::Json => serde_json::to_string_pretty(&record_list)?,
    //     OutputFormat::Yaml => serde_yaml::to_string(&record_list)?,
    //     OutputFormat::Toml => toml::to_string(&record_list)?,
    // };

    std::fs::write(output_file, content)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    fn vec_to_toml(vec: Vec<(&str, &str)>) -> String {
        let map: HashMap<&str, &str> = vec.into_iter().collect();
        toml::to_string(&map).unwrap()
    }
    #[test]
    fn test_toml() {
        // use serde::Serialize;
        // use toml;
        //
        // #[derive(Serialize)]
        // struct KeyValuePair {
        //     key: String,
        //     value: String,
        // }
        //
        // let pairs: Vec<(&str, &str)> = vec![("name", "Alice"), ("city", "Paris")];
        //
        // let kv_pairs: Vec<KeyValuePair> = pairs
        //     .into_iter()
        //     .map(|(k, v)| KeyValuePair {
        //         key: k.to_string(),
        //         value: v.to_string(),
        //     })
        //     .collect();
        //
        // let toml_str = toml::to_string(&kv_pairs).unwrap();
        // println!("{}", toml_str);

        let data = vec![("name", "Alice"), ("age", "30"), ("city", "New York")];

        let toml_string = vec_to_toml(data);
        println!("{}", toml_string);
    }
}

#[cfg(test)]
mod test_all_toml {
    use super::*;
    // 方法 1: 数组表格式
    fn vec_to_toml(data: Vec<Vec<(String, String)>>) -> String {
        use toml::Value;
        let tables: Vec<Value> = data
            .into_iter()
            .map(|pairs| {
                let map: Map<String, Value> = pairs
                    .into_iter()
                    .map(|(k, v)| (k, Value::String(v)))
                    .collect();
                Value::Table(map)
            })
            .collect();

        let mut root = BTreeMap::new();
        root.insert("items".to_string(), Value::Array(tables));

        toml::to_string(&root).unwrap()
    }

    // 方法 2: 嵌套结构
    fn vec_to_toml_nested(data: Vec<Vec<(String, String)>>) -> String {
        let mut root = BTreeMap::new();

        for (idx, pairs) in data.into_iter().enumerate() {
            let map: Map<String, TomlValue> = pairs
                .into_iter()
                .map(|(k, v)| (k, TomlValue::String(v)))
                .collect();

            root.insert(format!("table_{}", idx), TomlValue::Table(map));
        }

        toml::to_string(&root).unwrap()
    }

    // 方法 3: 自定义根键名
    fn vec_to_toml_with_key(data: Vec<Vec<(String, String)>>, key: &str) -> String {
        let tables: Vec<TomlValue> = data
            .into_iter()
            .map(|pairs| {
                let map: Map<String, TomlValue> = pairs
                    .into_iter()
                    .map(|(k, v)| (k, TomlValue::String(v)))
                    .collect();
                TomlValue::Table(map)
            })
            .collect();

        let mut root = BTreeMap::new();
        root.insert(key.to_string(), TomlValue::Array(tables));

        toml::to_string(&root).unwrap()
    }

    #[test]
    fn test_vec_to_toml_basic() {
        let data = vec![
            vec![
                ("name".to_string(), "Alice".to_string()),
                ("age".to_string(), "30".to_string()),
            ],
            vec![
                ("name".to_string(), "Bob".to_string()),
                ("age".to_string(), "25".to_string()),
            ],
        ];

        let result = vec_to_toml(data);

        // 验证输出包含预期内容
        assert!(result.contains("[[items]]"));
        assert!(result.contains("name = \"Alice\""));
        assert!(result.contains("age = \"30\""));
        assert!(result.contains("name = \"Bob\""));
        assert!(result.contains("age = \"25\""));
    }

    #[test]
    fn test_vec_to_toml_empty() {
        let data: Vec<Vec<(String, String)>> = vec![];
        let result = vec_to_toml(data);

        assert!(result.contains("items = []"));
    }

    #[test]
    fn test_vec_to_toml_single_item() {
        let data = vec![vec![("key".to_string(), "value".to_string())]];

        let result = vec_to_toml(data);

        assert!(result.contains("[[items]]"));
        assert!(result.contains("key = \"value\""));
    }

    #[test]
    fn test_vec_to_toml_multiple_fields() {
        let data = vec![vec![
            ("name".to_string(), "Alice".to_string()),
            ("age".to_string(), "30".to_string()),
            ("city".to_string(), "New York".to_string()),
            ("country".to_string(), "USA".to_string()),
        ]];

        let result = vec_to_toml(data);

        assert!(result.contains("name = \"Alice\""));
        assert!(result.contains("age = \"30\""));
        assert!(result.contains("city = \"New York\""));
        assert!(result.contains("country = \"USA\""));
    }

    #[test]
    fn test_vec_to_toml_special_characters() {
        let data = vec![vec![
            ("path".to_string(), "/usr/bin".to_string()),
            ("description".to_string(), "A \"quoted\" string".to_string()),
        ]];

        let result = vec_to_toml(data);

        assert!(result.contains("path = \"/usr/bin\""));
        assert!(result.contains("description"));
    }

    #[test]
    fn test_vec_to_toml_nested_basic() {
        let data = vec![
            vec![
                ("name".to_string(), "Alice".to_string()),
                ("age".to_string(), "30".to_string()),
            ],
            vec![
                ("name".to_string(), "Bob".to_string()),
                ("age".to_string(), "25".to_string()),
            ],
        ];

        let result = vec_to_toml_nested(data);

        assert!(result.contains("[table_0]"));
        assert!(result.contains("[table_1]"));
        assert!(result.contains("name = \"Alice\""));
        assert!(result.contains("name = \"Bob\""));
    }

    #[test]
    fn test_vec_to_toml_nested_empty() {
        let data: Vec<Vec<(String, String)>> = vec![];
        let result = vec_to_toml_nested(data);

        // 空数据应该产生空的 TOML
        assert_eq!(result.trim(), "");
    }

    #[test]
    fn test_vec_to_toml_with_key_custom() {
        let data = vec![vec![("name".to_string(), "Alice".to_string())]];

        let result = vec_to_toml_with_key(data, "users");

        assert!(result.contains("[[users]]"));
        assert!(result.contains("name = \"Alice\""));
    }

    #[test]
    fn test_vec_to_toml_with_key_different_keys() {
        let data = vec![vec![("title".to_string(), "Book 1".to_string())]];

        let result1 = vec_to_toml_with_key(data.clone(), "books");
        let result2 = vec_to_toml_with_key(data, "products");

        assert!(result1.contains("[[books]]"));
        assert!(result2.contains("[[products]]"));
    }

    #[test]
    fn test_roundtrip_parsing() {
        let data = vec![vec![
            ("name".to_string(), "Alice".to_string()),
            ("age".to_string(), "30".to_string()),
        ]];

        let toml_string = vec_to_toml(data);

        // 验证生成的 TOML 可以被解析回来
        let parsed: toml::Value = toml::from_str(&toml_string).unwrap();

        assert!(parsed.get("items").is_some());
        assert!(parsed["items"].is_array());
    }

    #[test]
    fn test_unicode_strings() {
        let data = vec![
            vec![
                ("name".to_string(), "张三".to_string()),
                ("city".to_string(), "北京".to_string()),
            ],
            vec![
                ("name".to_string(), "李四".to_string()),
                ("city".to_string(), "上海".to_string()),
            ],
        ];

        let result = vec_to_toml(data);

        assert!(result.contains("张三"));
        assert!(result.contains("北京"));
        assert!(result.contains("李四"));
        assert!(result.contains("上海"));
    }

    #[test]
    fn test_empty_inner_vec() {
        let data = vec![
            vec![],
            vec![("name".to_string(), "Alice".to_string())],
            vec![],
        ];

        let result = vec_to_toml(data);

        // 应该有3个表，即使有些是空的
        assert_eq!(result.matches("[[items]]").count(), 3);
    }
}
