//
// Macros to cut down on boilerplate code to convert JsonValue
// objects to the native String, i32, u64, etc., rust types.
// (These could be improved on a lot!)
//

#[macro_export]
macro_rules! json_val_to_i32 {
    ( $json_path:literal , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: i32 = $receiving_variable_default_value;
        let json_path = format!($json_path);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            if let Some(inner_val) = jsonval.as_i32() {
                $receiving_variable = inner_val;
            }
        }
    }
}

#[macro_export]
macro_rules! json_val_to_i64 {
    ( $json_path:literal , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: i64 = $receiving_variable_default_value;
        let json_path = format!($json_path);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            if let Some(inner_val) = jsonval.as_i64() {
                $receiving_variable = inner_val;
            }
        }
    }
}

#[macro_export]
macro_rules! json_val_to_string {
    ( $json_path:literal , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: String = $receiving_variable_default_value.to_string();
        let json_path = format!($json_path);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            $receiving_variable = jsonval.to_string();
            if $receiving_variable.len()>2 {
                $receiving_variable = $receiving_variable[1..$receiving_variable.len()-1].to_string();
            }
        }
    }
}

#[macro_export]
macro_rules! json_val_to_string_with_formatter {
    ( $json_path:literal # $arg0:ident , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: String = $receiving_variable_default_value.to_string();
        let json_path = format!($json_path, $arg0);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            $receiving_variable = jsonval.to_string();
            if $receiving_variable.len()>2 {
                $receiving_variable = $receiving_variable[1..$receiving_variable.len()-1].to_string();
            }
        }
    }
}

#[macro_export]
macro_rules! json_val_to_i32_with_formatter {
    ( $json_path:literal # $arg0:ident , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: i32 = $receiving_variable_default_value;
        let json_path = format!($json_path, $arg0);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            if let Ok(inner_val) = jsonval.to_string().parse::<i32>() {
                $receiving_variable = inner_val;
            }
        }
    }
}

#[macro_export]
macro_rules! json_val_to_u64 {
    ( $json_path:literal , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: u64 = $receiving_variable_default_value;
        let json_path = format!($json_path);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            if let Some(inner_val) = jsonval.as_u64() {
                $receiving_variable = inner_val;
            }
        }
    }
}
