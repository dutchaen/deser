pub fn convert_to_pascal_case(text: &str) -> String {
    let mut capitalize = false;
    let mut result = String::new();
    for (index, char) in text.chars().enumerate() {
        if char == '_' {
            capitalize = true;
            continue;
        }

        if index == 0 || capitalize {
            result.push(char.to_ascii_uppercase());
            capitalize = false;
            continue;
        }

        result.push(char);
        
    }
    return result;
}

pub fn golang_determine_array_type(val: &Vec<serde_json::Value>) -> &'static str {
    let mut result: &'static str= "";
    for item in val.iter() {
        if item.is_string() {
            if result.is_empty() {
                result = "[]string";  
            }
            else if result != "[]string" {
                result = "[]interface{}";
                break;
            }
        }
        else if item.is_boolean() {
            if result.is_empty() {
                result = "[]bool";  
            }
            else if result != "[]bool" {
                result = "[]interface{}";
                break;
            }
        }
        else if item.is_number() {
            if result.is_empty() {
                result = "[]float64";  
            }
            else if result != "[]float64{}" {
                result = "[]interface{}";
                break;
            }
        }
        else {
            return "[]interface{}";
        }
    }

    return result;

} 

pub fn rust_determine_array(val: &Vec<serde_json::Value>) -> &'static str {

    let mut result: &'static str= "";
    for item in val.iter() {
        if item.is_string() {
            if result.is_empty() {
                result = "Vec<String>";  
            }
            else if result != "Vec<String>" {
                result = "Vec<serde_json::Value>";
                break;
            }
        }
        else if item.is_boolean() {
            if result.is_empty() {
                result = "Vec<bool>";  
            }
            else if result != "Vec<bool>" {
                result = "Vec<serde_json::Value>";
                break;
            }
        }
        else if item.is_number() {
            if result.is_empty() {
                result = "Vec<f64>";  
            }
            else if result != "Vec<f64>" {
                result = "Vec<serde_json::Value>";
                break;
            }
        }
        else {
            return "Vec<serde_json::Value>";
        }
    }

    return result;
} 

pub fn csharp_determine_array(val: &Vec<serde_json::Value>) -> &'static str {
    let mut result: &'static str= "";
    for item in val.iter() {
        if item.is_string() {
            if result.is_empty() {
                result = "IEnumerable<String>";  
            }
            else if result != "IEnumerable<string>" {
                result = "IEnumerable<JObject>";
                break;
            }
        }
        else if item.is_boolean() {
            if result.is_empty() {
                result = "IEnumerable<bool>";  
            }
            else if result != "IEnumerable<bool>" {
                result = "IEnumerable<JObject>";
                break;
            }
        }
        else if item.is_number() {
            if result.is_empty() {
                result = "IEnumerable<double>";  
            }
            else if result != "IEnumerable<double>" {
                result = "IEnumerable<JObject>";
                break;
            }
        }
        else {
            return "IEnumerable<JObject>";
        }
    }

    return result;
}
