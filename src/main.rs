use graphler::parser::parse;
use serde_json;
use serde_json::to_string_pretty;
// use graphler::writer::write_to_file;

fn main() {
    let parsed = parse("src/etc").expect("parsing failed");
    let tmp = to_string_pretty(&parsed).expect("ooops!");
    println!("{}", tmp);

}
