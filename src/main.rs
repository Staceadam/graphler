use graphler::parser::parse;
use graphler_core::writer::write_to_file;

fn main() {
    let parsed = parse("src/etc/multiple/").expect("parsing failed");
    write_to_file(parsed, "graphler.json")
}
