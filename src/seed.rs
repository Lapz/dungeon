use sha2::{ Sha256, Digest };

pub fn create_hash(text:&str) -> String {
   let mut hasher = Sha256::default();
   hasher.input(text.as_bytes());

   format!("{:x}",hasher.result())
}