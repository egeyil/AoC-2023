fn main() {
    println!("hello world");
}

fn process() -> String {
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(contents, process());
    }
}