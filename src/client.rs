//#[cfg(features="client")]
pub mod client {  
    use Backend::example;

    pub fn client_main() {
        example();
        println!("Hello, world!");
    }
}