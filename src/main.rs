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

pub struct FnSyntax {
    qualifer: Vec<String>,
    identifier: String,
    generic_param: String,
    params: String,
    return_type: String,
    return_where_clause: String,
}

fn main() {
    let rs_fn = String::from(
        "pub async fn produce_message<'a, P, K, H>(
            producer: &FutureProducer,
            topic: &str,
            payload: P,
            key: K,
            header: Option<&H>,
        ) -> Result<(), (KafkaError, OwnedMessage)>",
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

fn parse_id(rs_fn_id: &str) -> (String, bool) {
    let mut tx_fn_id = Vec::new();

    // remove paren and split param
    let rs_fn_qualifier_and_else = rs_fn_id.split_once("fn").unwrap();

    // push visibility
    if rs_fn_qualifier_and_else.0.contains("pub") {
        tx_fn_id.push(String::from("export"));
    }
    // push function keyword
    tx_fn_id.push(String::from("function"));
    // push id and generic
    tx_fn_id.push(rs_fn_qualifier_and_else.1.trim().to_string());
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
            ts_fn_param_name_vec.push(param_name_and_type.0);
            ts_fn_param_type_vec.push(param_name_and_type.1);
        }
    });
    println!("{:?}", rs_fn_param_vec);

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
