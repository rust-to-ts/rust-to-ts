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
    let [id, param, return_type]: [&str; 3] = parse_fn(&rs_fn);
    let (mut rs_fn_id, is_async) = parse_id(&id);
    let rs_fn_param = parse_param(&param);
    let rs_fn_return_type = parse_return_type(&return_type);

    println!("{}", rs_fn_id);
    println!("{}", rs_fn_param);
    println!("{}", rs_fn_return_type);

    // push param
    rs_fn_id.push_str(" ");
    rs_fn_id.push_str("(");
    rs_fn_id.push_str(&rs_fn_param);
    rs_fn_id.push_str(")");

    // push return type
    rs_fn_id.push_str(" ");
    rs_fn_id.push_str(":");
    rs_fn_id.push_str(" ");
    // if async, wrap in Promise
    if is_async {
        rs_fn_id.push_str("Promise<");
    }
    rs_fn_id.push_str(&rs_fn_return_type);
    // if async, wrap in Promise
    if is_async {
        rs_fn_id.push_str(">");
    }
    rs_fn_id.push_str(";");

    println!("{}", rs_fn_id);
}

fn parse_fn(rs_fn: &str) -> [&str; 3] {
    // separate fn before params
    let rs_fn_id_and_else = rs_fn.split_once('(').unwrap();
    // separate params and return type
    let rs_fn_param_and_return = rs_fn_id_and_else.1.split_once("->").unwrap();
    return [
        rs_fn_id_and_else.0.trim(),
        rs_fn_param_and_return.0.trim(),
        rs_fn_param_and_return.1.trim(),
    ];
}

pub fn parse_id(rs_fn_id: &str) -> (String, bool) {
    let mut tx_fn_id = Vec::new();

    // remove paren and split param
    let rs_fn_qualifier_and_else = rs_fn_id.split_once("fn").unwrap();

    // push visibility
    if is_public_visibility(rs_fn_qualifier_and_else.0) {
        tx_fn_id.push(String::from("export"));
    }
    // push function keyword
    tx_fn_id.push(String::from("function"));
    // parse generic params if exists
    if is_generic_params_exist(rs_fn_qualifier_and_else.1) {
        // separate function name and generic params
        let rs_fn_name_and_generic: (&str, &str) =
            rs_fn_qualifier_and_else.1.trim().split_once('<').unwrap();
        // push function name first
        tx_fn_id.push(rs_fn_name_and_generic.0.to_string());
        let parsed_generic_params: String = parse_generic_type(rs_fn_name_and_generic.1);
        // push generic params
        tx_fn_id.push(String::from("<"));
        tx_fn_id.push(parsed_generic_params);
        tx_fn_id.push(String::from(">"));
    } else {
        tx_fn_id.push(rs_fn_qualifier_and_else.1.trim().to_string());
    }
    return (
        tx_fn_id.join(" "),
        rs_fn_qualifier_and_else.0.contains("async"),
    );
}

fn parse_param(rs_fn_param: &str) -> String {
    let mut ts_fn_param_name_vec: Vec<&str> = Vec::new();
    let mut ts_fn_param_type_vec: Vec<&str> = Vec::new();
    // remove paren and split param
    let mut rs_fn_param_vec: Vec<&str> = rs_fn_param[..rs_fn_param.len() - 1].split(',').collect();
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

fn parse_return_type(rs_fn_return_type: &str) -> String {
    return String::from(rs_fn_return_type);
}

fn parse_generic_type(rs_generic_type: &str) -> String {
    let mut ts_generic_type_vec: Vec<&str> = Vec::new();

    let rs_generic_type_vec: Vec<&str> = rs_generic_type
        .split_once('>') // remove generic '>'
        .unwrap()
        .0
        .split(',')
        .collect();
    rs_generic_type_vec.iter().for_each(|generic_scalar| {
        let generic_type = generic_scalar.trim();
        if !generic_scalar.is_empty() {
            if !is_lifetime_type(generic_type) {
                ts_generic_type_vec.push(remove_attributes_and_mut_on_param(&generic_type));
            }
        }
    });

    return ts_generic_type_vec.join(", ");
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
