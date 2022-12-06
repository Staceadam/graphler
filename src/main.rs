use graphler::parser::parse;
use graphler::writer::write_to_file;

fn main() {
    let parsed = parse("src/etc").expect("parsing failed");
    write_to_file(parsed, "tmp.json");

}
