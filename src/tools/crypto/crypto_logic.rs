use sha2::{Digest, Sha256};
use md5;

pub struct CryptoHandler {}

impl CryptoHandler {
    pub fn new() -> CryptoHandler {
        CryptoHandler {}
    }

    pub fn hash_sha256(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn hash_md5(data: &str) -> String {
        let result = md5::compute(data);
        format!("{:x}", result)
    }

    pub fn stress_test(num_iterations: usize, data: &str) {
        for _ in 0..num_iterations {
            let _ = CryptoHandler::hash_sha256(data);
            let _ = CryptoHandler::hash_md5(data);
        }
    }
}
