mod password_generator;

fn main() {
    println!(
        "{}",
        password_generator::generate(
            None,
            None,
            None,
            None,
            None,
            Option::Some(Box::new(vec!['\0']))
        )
        .unwrap()
    );
}
