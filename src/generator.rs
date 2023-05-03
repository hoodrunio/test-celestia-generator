use hex::encode;
use rand::{rngs::ThreadRng, Rng};

pub struct PayForBlobGen {
    pub rand: ThreadRng,
}

impl PayForBlobGen {
    pub fn new() -> Self {
        Self {
            rand: rand::thread_rng(),
        }
    }

    fn hexer(&self, bytes: &[u8]) -> String {
        encode(bytes)
    }

    pub fn namespace_id(&mut self) -> String {
        let mut random_bytes = [0u8; 8];
        self.rand.fill(&mut random_bytes); // Generate 8 random bytes
        self.hexer(&random_bytes) // Encode the bytes as a hex string and return it
    }

    pub fn message(&mut self, length: usize) -> String {
        assert!(length <= 100, "Message length should be up to 100 bytes");

        let mut random_bytes = vec![0u8; length];
        self.rand.fill(&mut random_bytes[..]); // Generate random bytes with the given length
        self.hexer(&random_bytes) // Encode the bytes as a hex string and return it
    }
}

impl Default for PayForBlobGen {
    fn default() -> Self {
        Self::new()
    }
}
