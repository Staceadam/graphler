use graphler::parser::parse;

fn main() {
    let parsed = parse("src/etc/multiple").expect("parsing failed");
}
