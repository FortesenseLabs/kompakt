mod balmer;
mod enwik9_preproc;

fn main() {
    println!("Kompakt!");

    let encoded_symbols = vec![0x421C_7EC3, 0x000B_8ED1];
    let symbols = vec![23i32, -15, 78, 43, -69]; // [23i32, -15, 78, 43, -69]

    // assert_eq!(encode_sample_data(), [0x421C_7EC3, 0x000B_8ED1]);
    if balmer::example::encode_sample_data() == encoded_symbols {
        println!("Encoded!");
    }

    // assert_eq!(decode_sample_data(encoded_symbols), decoded_symbols);
    if balmer::example::decode_sample_data(encoded_symbols) == symbols {
        println!("Decoded!");
    }

    // run basic example
    balmer::example::basic_example()
}

// https://en.wikipedia.org/wiki/PAQ
// https://docs.rs/constriction/0.3.0/constriction/
