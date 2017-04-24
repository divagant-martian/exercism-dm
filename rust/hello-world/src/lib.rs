pub fn hello(name: Option<&str>) -> String {
    if name.is_some(){
        "Hello, ".to_string() + name.unwrap() + "!"
    }
    else {
        "Hello, World!".to_string()
    }
    
}
