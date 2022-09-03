use graphler::parser::parse;

fn main() {
    parse("src/etc").expect("parsing failed");
}
