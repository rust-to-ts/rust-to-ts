// TS SYNTAX
const TS_PRIMITIVE_TYPE: [&str; 5] = ["number", "string", "boolean", "null", "undefined"];
const TS_OBJECT_TYPE: [&str; 5] = ["class", "enum", "type", "interface", "object"];
const TS_KEYWORD_TYPE: [&str; 6] = [
    "abstract",
    "extends",
    "public",
    "private",
    "protected",
    "static",
];
const TS_FUCTION_TYPE: [&str; 2] = ["Function", "void"];
// RS SYNTAX
const RS_FN_QUALIFIER: [&str; 4] = ["const", "async", "unsafe", "extern"];
const RS_FN_VISIBILITY: [&str; 1] = ["pub"];

use std::collections::HashMap;

struct FnSyntax {
    qualifer: Vec<String>,
    identifier: String,
    generic_param: String,
    params: String,
    return_type: String,
    return_where_clause: String,
}

fn main() {
    let rs_fn = String::from(
        "pub async fn test<'a, P, K, H>(
            alice: &TestParamType,
            bob: &str,
            chris: P,
            mut hyunhum: K,
            #[warn(unused_mut)] mut minwoo: Option<&H>,
        ) -> Result<(), (TestError, TestReturnType)>
        where
            P: 'a + ToBytes,
            K: 'a + ToBytes,
            H: 'a + ToBytes,
        ",
    );
    let [rs_fn_header, rs_fn_input, rs_fn_output]: [&str; 3] = parse_fn(&rs_fn);
    let (ts_fn_header, is_async, mut generic_params_map) = parse_fn_header(&rs_fn_header);
    let ts_fn_input = parse_fn_input(&rs_fn_input);
    let ts_fn_output = parse_fn_output(&rs_fn_output, &mut generic_params_map);

    println!("{}", ts_fn_header);
    println!("{}", ts_fn_input);
    println!("{}", ts_fn_output);

    // declare ts function
    let mut ts_fn = ts_fn_header;

    // push generic
    if !generic_params_map.is_empty() {
        ts_fn.push_str("<");
        ts_fn.push_str(&transform_generic_params(&generic_params_map));
        ts_fn.push_str(">");
    }

    // push param
    ts_fn.push_str(" ");
    ts_fn.push_str("(");
    ts_fn.push_str(&ts_fn_input);
    ts_fn.push_str(")");

    // push return type
    ts_fn.push_str(" ");
    ts_fn.push_str(":");
    ts_fn.push_str(" ");
    // if async, wrap in Promise
    if is_async {
        ts_fn.push_str("Promise<");
    }
    ts_fn.push_str(&ts_fn_output);
    // if async, wrap in Promise
    if is_async {
        ts_fn.push_str(">");
    }
    ts_fn.push_str(";");

    println!("{}", ts_fn);
}

fn parse_fn(rs_fn: &str) -> [&str; 3] {
    // separate fn before params
    let (rs_fn_header, remainder) = rs_fn.split_once('(').unwrap();
    // separate params and return type
    let (rs_fn_input, rs_fn_output) = remainder.split_once("->").unwrap();
    return [
        rs_fn_header.trim(),
        rs_fn_input.trim(),
        rs_fn_output.trim(),
    ];
}

// To do: fn name must be parsed as fn_decl
pub fn parse_fn_header(rs_fn_header: &str) -> (String, bool, HashMap<String, Vec<String>>) {
    let mut ts_fn_header = Vec::new();

    // split qualifier
    let (rs_fn_qualifier, remainder) = rs_fn_header.split_once("fn").unwrap();

    // push visibility
    if is_public_visibility(rs_fn_qualifier) {
        ts_fn_header.push(String::from("export"));
    }
    // push function keyword
    ts_fn_header.push(String::from("function"));

    // parse generic params if exists
    let mut generic_params_map: HashMap<String, Vec<String>> = HashMap::new();
    if is_generic_params_exist(remainder) {
        // separate function name and generic params
        let (rs_fn_ident, mut rs_fn_generic_params): (&str, &str) =
            remainder.trim().split_once('<').unwrap();
        rs_fn_generic_params = rs_fn_generic_params.split_once('>').unwrap().0;
        parse_fn_generic_params(rs_fn_generic_params, &mut generic_params_map);

        // push function name
        ts_fn_header.push(rs_fn_ident.to_string());
    } else {
        ts_fn_header.push(remainder.trim().to_string());
    }
    return (
        ts_fn_header.join(" "),
        rs_fn_qualifier.contains("async"),
        generic_params_map,
    );
}

fn parse_fn_input(rs_fn_input: &str) -> String {
    let mut ts_fn_param_name_vec: Vec<&str> = Vec::new();
    let mut ts_fn_param_type_vec: Vec<&str> = Vec::new();
    // remove paren and split param
    let mut rs_fn_param_vec: Vec<&str> = rs_fn_input[..rs_fn_input.len() - 1].split(',').collect();
    // trim each param
    rs_fn_param_vec.iter_mut().for_each(|param_scalar| {
        *param_scalar = param_scalar.trim();
        if !param_scalar.is_empty() {
            let param_name_and_type = param_scalar.split_once(':').unwrap();
            ts_fn_param_name_vec.push(remove_attributes_and_mut_on_param(param_name_and_type.0));
            ts_fn_param_type_vec.push(param_name_and_type.1);
        }
    });
    let mut ts_fn_param = String::new();

    for i in 0..ts_fn_param_name_vec.len() {
        ts_fn_param.push_str(ts_fn_param_name_vec[i]);
        ts_fn_param.push_str(":");
        ts_fn_param.push_str(ts_fn_param_type_vec[i]);
        ts_fn_param.push_str(", ");
    }

    return ts_fn_param;
}

fn parse_fn_output(rs_fn_output: &str, generic_params_map: &mut HashMap<String, Vec<String>>) -> String {
    let ts_fn_output: &str;
    match rs_fn_output.split_once("where") {
        Some((fn_output, where_clause)) => {
            ts_fn_output = fn_output;
            parse_fn_generic_params(where_clause, generic_params_map);
        },
        None => {
            ts_fn_output = rs_fn_output;
        }
    }
    return String::from(ts_fn_output);
}

fn parse_fn_generic_params(rs_fn_generic_params: &str, generic_params_map: &mut HashMap<String, Vec<String>>) {
    let rs_fn_generic_param_vec: Vec<&str> = rs_fn_generic_params
        .split(',')
        .collect();

    rs_fn_generic_param_vec.iter().for_each(|generic_scalar| {
        let generic_param = generic_scalar.trim();
        if !generic_param.is_empty() {
            if !is_lifetime_type(generic_param) {
                let (ident, generic_bounds) = match generic_param.split_once(':') {
                    Some(tuple) => tuple,
                    None => {
                        generic_params_map.entry(String::from(generic_param)).or_insert(Vec::new());
                        return;
                    }
                };
                let mut generic_bound_vec: Vec<&str> = generic_bounds.split('+').collect();
                let mut cloned_generic_bound_vec: Vec<String> = Vec::new();
                for mut generic_bound in generic_bound_vec {
                    generic_bound = generic_bound.trim();
                    if !is_lifetime_type(generic_bound) { cloned_generic_bound_vec.push(String::from(generic_bound)); };
                }
                let generic_bounds_entry = generic_params_map.entry(String::from(ident)).or_insert(Vec::new());
                (*generic_bounds_entry).append(&mut cloned_generic_bound_vec);
            }
        }
    });
}

fn transform_generic_params(generic_params_map: &HashMap<String, Vec<String>>) -> String {
    let mut ts_fn_generic_param_vec: Vec<String> = Vec::new();
    for (key, value) in generic_params_map {
        let mut ts_fn_generic_param = String::from(remove_attributes_and_mut_on_param(key));
        ts_fn_generic_param.push_str(" extends ");
        ts_fn_generic_param.push_str(&value.join(" & "));
        ts_fn_generic_param_vec.push(ts_fn_generic_param);
    }
    return String::from(ts_fn_generic_param_vec.join(", "));
}

fn is_reference_type(rs_type: &str) -> bool {
    return rs_type.starts_with('&');
}

fn is_dereference_type(rs_type: &str) -> bool {
    return rs_type.starts_with('*');
}

fn is_lifetime_type(rs_type: &str) -> bool {
    // ex) 'a, 'b: a, 'static, '_
    // must analyze what where contains later
    return rs_type.starts_with('\'');
}

fn is_generic_params_exist(rs_syntax: &str) -> bool {
    return rs_syntax.contains('<');
}

fn is_public_visibility(rs_syntax: &str) -> bool {
    return rs_syntax.starts_with("pub");
}

fn is_self_type(rs_type: &str) -> bool {
    // the type of the current object
    // It may appear either in a trait or an impl
    return rs_type.eq("Self");
}

fn is_self_first_param(rs_type: &str) -> bool {
    // used in a trait or an impl for the first argument of a method
    return rs_type.eq("self");
}

fn remove_attributes_and_mut_on_param(rs_fn_param_name: &str) -> &str {
    // 1. remove macro attribute #[]
    // 2. remove "mut"
    let rs_fn_param_name_vec: Vec<&str> = rs_fn_param_name.split(' ').collect();
    // last element is name
    return rs_fn_param_name_vec[rs_fn_param_name_vec.len() - 1];
}
