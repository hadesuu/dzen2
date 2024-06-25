#[ic_cdk::query]
fn greet(name: String, numer: i8) -> String {
    format!("Hello, {} {}!", name, numer)
}
