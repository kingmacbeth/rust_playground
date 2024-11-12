use sha2::{Sha256, Digest};

fn main() {
    // Preimage (input)
    let preimage = "hello";

    // Create a Sha256 object
    let mut hasher = Sha256::new();

    // Write input message (preimage) into the hasher
    hasher.update(preimage);

    // Read hash digest and convert the result to a hex string
    let result = hasher.finalize();

    println!("Preimage: {}", preimage);
    println!("Hash: {:x}", result);
}
