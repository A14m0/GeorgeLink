//#[cfg(features="server")]

pub mod server {
    use Backend::example;
    
    
    pub fn server_main() {
        example();
        println!("Hello, world!");
    }
}