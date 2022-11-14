#[no_mangle]
pub extern "C" fn see_ffi_in_action() {
    println!("Congrats! You have successfully invked Rust shared library from a C program");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
