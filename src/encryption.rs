use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader};

pub fn get_encryption(
    key: &str,
    cert: &str
) -> ServerConfig {

    // Start at the root directory
    let root_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut cert_path = root_path.clone();
    let mut key_path = root_path.clone();

    cert_path.push_str(cert.as_ref());
    key_path.push_str(key);

    // load ssl keys
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let cert_file = &mut BufReader::new(File::open(cert_path).expect("No cert found."));
    let key_file = &mut BufReader::new(File::open(key_path).expect("No key found."));

    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

