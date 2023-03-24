use constriction::stream::{model::DefaultLeakyQuantizer, stack::DefaultAnsCoder, Decode};
use probability::distribution::Gaussian;

// Let's use an ANS Coder in this example. Constriction also provides a Range
// Coder, a Huffman Coder, and an experimental new "Chain Coder".
fn encode_sample_data() -> Vec<u32> {
    // Create an empty ANS Coder with default word and state size:
    let mut coder = DefaultAnsCoder::new();

    // Some made up data and entropy models for demonstration purpose:
    let symbols = [23i32, -15, 78, 43, -69];
    let means = [35.2, -1.7, 30.1, 71.2, -75.1];
    let stds = [10.1, 25.3, 23.8, 35.4, 3.9];

    // Create an adapter that integrates 1-d probability density functions over bins
    // `[n - 0.5, n + 0.5)` for all integers `n` from `-100` to `100` using fixed point
    // arithmetic with default precision, guaranteeing a nonzero probability for each bin:
    let quantizer = DefaultLeakyQuantizer::new(-100..=100);

    // Encode the data (in reverse order, since ANS Coding operates as a stack):
    coder
        .encode_symbols_reverse(
            symbols
                .iter()
                .zip(&means)
                .zip(&stds)
                .map(|((&sym, &mean), &std)| (sym, quantizer.quantize(Gaussian::new(mean, std)))),
        )
        .unwrap();

    // Retrieve the compressed representation (filling it up to full words with zero bits).
    // coder.into_compressed().unwrap()
    return coder.into_compressed().unwrap();
}

fn decode_sample_data(compressed: Vec<u32>) -> Vec<i32> {
    // Create an ANS Coder with default word and state size from the compressed data:
    // (ANS uses the same type for encoding and decoding, which makes the method very flexible
    // and allows interleaving small encoding and decoding chunks, e.g., for bits-back coding.)
    let mut coder = DefaultAnsCoder::from_compressed(compressed).unwrap();

    // Same entropy models and quantizer we used for encoding:
    let means = [35.2, -1.7, 30.1, 71.2, -75.1];
    let stds = [10.1, 25.3, 23.8, 35.4, 3.9];
    let quantizer = DefaultLeakyQuantizer::new(-100..=100);

    // Decode the data:
    coder
        .decode_symbols(
            means
                .iter()
                .zip(&stds)
                .map(|(&mean, &std)| quantizer.quantize(Gaussian::new(mean, std))),
        )
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn main() {
    println!("Kompakt!");
    //
    let encoded_symbols = vec![0x421C_7EC3, 0x000B_8ED1];
    let decoded_symbols = [23, -15, 78, 43, -69]; // [23i32, -15, 78, 43, -69]
                                                  // assert_eq!(encode_sample_data(), [0x421C_7EC3, 0x000B_8ED1]);
    if encode_sample_data() == encoded_symbols {
        println!("Encoded!");
    }
    //
    // assert_eq!(decode_sample_data(encoded_symbols), decoded_symbols);
    if decode_sample_data(encoded_symbols) == decoded_symbols {
        println!("Decoded!");
    }
}
