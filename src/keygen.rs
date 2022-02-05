use rcgen;

pub fn _get_cert() {
    let cert = rcgen::generate_simple_self_signed(
        vec![
            "localhost".to_string(),
            "example.world".to_string()
        ]
    ).unwrap();

    println!("{}", cert.serialize_pem().unwrap());
    println!("{}", cert.serialize_private_key_pem());
}