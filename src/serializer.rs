use std::collections::HashMap;
use crate::helper::{self, golang_determine_array_type, rust_determine_array, csharp_determine_array};


pub fn rust(json_object: &serde_json::Value, name: &str, result: &mut String, used_structs: &mut Vec<String>) {
    let mut values: HashMap<String, &serde_json::Value> = HashMap::new();
    result.push_str(&format!("\r\n\r\n#[derive(Serialize, Deserialize)]\r\npub struct {} {{\r\n", name));

    if json_object.is_object() {
        if let Some(object) = json_object.as_object() {
            for (key, value) in object.iter() {
                let pascal_cased = helper::convert_to_pascal_case(&key);
                if value.is_string() {
                    result.push_str(&format!("\tpub {}: String,\r\n", key));
                }
                else if value.is_boolean() {
                    result.push_str(&format!("\tpub {}: bool,\r\n", key));
                }
                else if value.is_number() {
                    result.push_str(&format!("\tpub {}: f64,\r\n", key));
                }
                else if value.is_null() {
                    result.push_str(&format!("\tpub {}: serde_json::Value,\r\n", key));
                }
                else if value.is_object() {
                    values.insert(pascal_cased.clone(), value);
                    result.push_str(&format!("\tpub {}: {},\r\n", key, pascal_cased));
                }
                else if value.is_array() {
                    let mut noun_pascal = pascal_cased.clone();
                    noun_pascal.pop();

                    let arr = value.as_array().unwrap();
                    if let Some(element) = arr.iter().next() {
                        if element.is_object() {
                            values.insert(noun_pascal.clone(), element);
                            result.push_str(&format!("\tpub {}: Vec<{}>,\r\n", key, noun_pascal));
                        }
                        else {
                            let array_type = rust_determine_array(arr);
                            result.push_str(&format!("\tpub {}: {},\r\n", key, array_type));
                        }
                    }
                    else {
                        result.push_str(&format!("\tpub {}: Vec<serde_json::Value>,\r\n", key));
                    }
                }
            }
        }
    }
    else if json_object.is_array() {
        let pascal_cased = helper::convert_to_pascal_case(&name);
        let mut noun_pascal = pascal_cased.clone();
        noun_pascal.pop();

        let arr = json_object.as_array().unwrap();
        if let Some(element) = arr.iter().next() {
            match element.is_object() {
                true => {
                    values.insert(noun_pascal.clone(), element);
                    result.push_str(&format!("\tpub {}: Vec<{}>,\r\n", name, noun_pascal));
                },
                false => panic!("The initial JSON object given is an array of a general type [{}]. Please deserialize it into its proper type.", rust_determine_array(arr))   
            };
        }
        else {
            result.push_str(&format!("\tpub {}: Vec<serde_json::Value>,\r\n", name));
        }
    }

    result.push('}');

    for (struct_name, value) in values.iter() {
        if used_structs.contains(struct_name) {
            continue;
        }

        rust(&value, &struct_name, result, used_structs);
        used_structs.push(struct_name.clone());
    }
}


pub fn golang(json_object: &serde_json::Value, name: &str, result: &mut String, used_structs: &mut Vec<String>) {
    let mut values: HashMap<String, &serde_json::Value> = HashMap::new();

    result.push_str(&format!("\r\n\r\ntype {} struct {{\r\n", name));

    if json_object.is_object() {
        if let Some(object) = json_object.as_object() {
            for (key, value) in object.iter() {
                let pascal_cased = helper::convert_to_pascal_case(&key);
                if value.is_string() {
                    result.push_str(&format!("\t{} string `json:\"{}\"`\r\n", pascal_cased, key));
                }
                else if value.is_boolean() {
                    result.push_str(&format!("\t{} bool `json:\"{}\"`\r\n", pascal_cased, key));
                }
                else if value.is_number() {
                    result.push_str(&format!("\t{} float64 `json:\"{}\"`\r\n", pascal_cased, key));
                }
                else if value.is_null() {
                    result.push_str(&format!("\t{} interface{{}} `json:\"{}\"`\r\n", pascal_cased, key));
                }
                else if value.is_object() {
                    values.insert(pascal_cased.clone(), value);
                    result.push_str(&format!("\t{} {} `json:\"{}\"`\r\n", pascal_cased, pascal_cased, key));
                }
                else if value.is_array() {
                    let mut noun_pascal = pascal_cased.clone();
                    noun_pascal.pop();

                    let arr = value.as_array().unwrap();
                    if let Some(element) = arr.iter().next() {
                        if element.is_object() {
                            values.insert(noun_pascal.clone(), element);
                            result.push_str(&format!("\t{} []{} `json:\"{}\"`\r\n", pascal_cased, noun_pascal, key));
                        }
                        else {
                            let array_type = golang_determine_array_type(arr);
                            result.push_str(&format!("\t{} {} `json:\"{}\"`\r\n", pascal_cased, array_type, key));
                        }
                    }
                    else {
                        result.push_str(&format!("\t{} []interface{{}} `json:\"{}\"`\r\n", pascal_cased, key));
                    }
                }
            }
        }
    } else if json_object.is_array() {
        let pascal_cased = helper::convert_to_pascal_case(&name);
        let mut noun_pascal = pascal_cased.clone();
        noun_pascal.pop();

        let arr = json_object.as_array().unwrap();
        if let Some(element) = arr.iter().next() {
            match element.is_object() {
                true => {
                    values.insert(noun_pascal.clone(), element);
                    result.push_str(&format!("\t{} []{} `json:\"{}\"`\r\n", pascal_cased, noun_pascal, name));
                }
                false => panic!("The initial JSON object given is an array of a general type [{}]. Please deserialize it into its proper type.", golang_determine_array_type(arr))
            };
        }
        else {
            result.push_str(&format!("\t{} []interface{{}} `json:\"{}\"`\r\n", pascal_cased, name));
        }
    }

    result.push('}');

    for (struct_name, value) in values.iter() {
        if used_structs.contains(struct_name) {
            continue;
        }

        golang(&value, &struct_name, result, used_structs);
        used_structs.push(struct_name.clone());
    }
}

pub fn csharp(json_object: &serde_json::Value, name: &str, result: &mut String, used_structs: &mut Vec<String>) {
    let mut values: HashMap<String, &serde_json::Value> = HashMap::new();

    result.push_str(&format!("\r\n\r\npublic class {}\r\n{{\r\n", name));

    if json_object.is_object() {
        if let Some(object) = json_object.as_object() {
            for (key, value) in object.iter() {
                let pascal_cased = helper::convert_to_pascal_case(&key);
                if value.is_string() {
                    result.push_str(&format!("\tpublic string {} {{ get; set; }} \r\n", pascal_cased));
                }
                else if value.is_boolean() {
                    result.push_str(&format!("\tpublic bool {} {{ get; set; }}\r\n", pascal_cased));
                }
                else if value.is_number() {
                    result.push_str(&format!("\tpublic double {} {{ get; set; }}\r\n", pascal_cased));
                }
                else if value.is_null() {
                    result.push_str(&format!("\tpublic object {} {{ get; set; }}\r\n", pascal_cased));
                }
                else if value.is_object() {
                    values.insert(pascal_cased.clone(), value);
                    result.push_str(&format!("\tpublic {} {} {{ get; set; }}\r\n", pascal_cased, pascal_cased));
                }
                else if value.is_array() {
                    let mut noun_pascal = pascal_cased.clone();

                    noun_pascal.pop();

                    let arr = value.as_array().unwrap();
                    if let Some(element) = arr.iter().next() {
                        if element.is_object() {
                            values.insert(noun_pascal.clone(), element);
                            result.push_str(&format!("\tpublic IEnumerable<{}> {} {{ get; set; }}\r\n", noun_pascal, pascal_cased));
                        }
                        else {
                            let array_type = csharp_determine_array(arr);
                            result.push_str(&format!("\tpublic {} {} {{ get; set; }}\r\n", array_type, pascal_cased));
                        }
                    }
                    else {
                        result.push_str(&format!("\tpublic IEnumerable<JObject> {} {{ get; set; }}\r\n", pascal_cased));
                    }
                }
            }
        }
    }
    else if json_object.is_array() {
        let pascal_cased = helper::convert_to_pascal_case(&name);
        let mut noun_pascal = pascal_cased.clone();
        noun_pascal.pop();

        let arr = json_object.as_array().unwrap();
        if let Some(element) = arr.iter().next() {
            match element.is_object() {
                true => {
                    values.insert(noun_pascal.clone(), element);
                    result.push_str(&format!("\tpublic IEnumerable<{}> {} {{ get; set; }}\r\n", noun_pascal, pascal_cased));
                },
                false => panic!("The initial JSON object given is an array of a general type [{}]. Please deserialize it into its proper type.", csharp_determine_array(arr))       
            };
        }
        else {
            result.push_str(&format!("\tpublic IEnumerable<JObject> {} {{ get; set; }}\r\n", pascal_cased));
        }
    }

    result.push('}');

    for (struct_name, value) in values.iter() {
        if used_structs.contains(struct_name) {
            continue;
        }

        csharp(&value, &struct_name, result, used_structs);
        used_structs.push(struct_name.clone());
    }
}
