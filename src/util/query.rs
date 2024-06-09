use regex::Captures;
use regex::Regex;

pub fn fix_query_param(url: String, var: String) -> String {
    let re = Regex::new(&(var.clone() + r"%5B(\w*)%5D")).unwrap();
    re.replace_all(&url, |caps: &Captures| format!("{}[{}]", var, &caps[1]))
        .to_string()
}

pub fn fix_regular_params(url: String) -> String {
    let fields = ["ids", "u64s", "strs", "strstrs", "bools"];
    let mut tmp = url;
    for field in fields {
        tmp = fix_query_param(tmp, field.to_string());
    }
    return tmp;
}
