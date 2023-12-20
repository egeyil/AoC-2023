fn main() {
    println!("{}", process());
}

fn process() -> String {
    "Hello World!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("1", process());
    }
}