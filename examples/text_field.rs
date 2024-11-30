use mevcard::text_field::text_field;

fn main() {
    let t = text_field(
        "properties,are,cool!,also\\,,backslashes: \\\\,and,semicolons: \\;".to_string(),
        ',',
    );
    println!("{}", t.join("\n"));
}
