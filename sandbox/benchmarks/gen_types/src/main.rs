use rand::Rng;

fn generate_int(name: &str, var_type: &str, max_value: usize) -> String {
    let value = rand::thread_rng().gen_range(1, max_value);
    format!(
        "let {name}: {var_type} = {value};",
        name = name,
        var_type = var_type,
        value = value,
    )
}
fn main() {
    let mut definitions: Vec<String> = Vec::new();

    let types: [(&str, usize); 2] = [
        ("u8", u8::MAX.into()),
        ("u16", u16::MAX.into()),
        // usize can't handle the rest
        // ("u32", u32::MAX.into()),
        // ("u64", u64::MAX.into()),
        // ("u128", u128::MAX.into()),
        // ("i8", i8::MAX.into()),
        // ("i16", i16::MAX.into()),
        // ("i32", i32::MAX.into()),
        // ("i64", i64::MAX.into()),
        // ("i128", i128::MAX.into()),
    ];

    for _ in 0..10000 {
        for t in types.iter() {
            definitions.push(generate_int("x", t.0, t.1.into()));
        }
    }

    println!(
        "{}",
        format!(
            "#[allow(unused_variables)]\n\
            fn main() {{\n\
                \t{definitions}\n\
            }}",
            definitions = definitions.join("\n\t")
        )
    );
}
