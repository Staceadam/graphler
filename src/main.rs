use graphler::parser::parse;

fn main() {
    let parsed = parse("src/etc").expect("parsing failed");
    // println!("{:#?}", parsed)
}
