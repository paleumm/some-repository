#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn hello_from_lib(message: &str) {
    println!("Printing Hello {} from library", message);
}
