use constriction::stream::{
    model::DefaultLeakyQuantizer, stack::DefaultAnsCoder, Decode,
};
use probability::distribution::Gaussian;

// Let's use an ANS Coder in this example. Constriction also provides a Range
// Coder, a Huffman Coder, and an experimental new "Chain Coder".
pub fn encode_sample_data() -> Vec<u32> {
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
        .encode_symbols_reverse(symbols.iter().zip(&means).zip(&stds).map(
            |((&sym, &mean), &std)| {
                (sym, quantizer.quantize(Gaussian::new(mean, std)))
            },
        ))
        .unwrap();

    // Retrieve the compressed representation (filling it up to full words with zero bits).
    // coder.into_compressed().unwrap()
    return coder.into_compressed().unwrap();
}

pub fn decode_sample_data(compressed: Vec<u32>) -> Vec<i32> {
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
            means.iter().zip(&stds).map(|(&mean, &std)| {
                quantizer.quantize(Gaussian::new(mean, std))
            }),
        )
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn basic_example() {
    // Let's use an ANS Coder in this example. Constriction also provides an Range
    // Coder, a Huffman Coder, and an experimental new "Chain Coder".
    let mut coder = DefaultAnsCoder::new();

    // Define some data and a sequence of entropy models. We use quantized Gaussians
    // here, but you could also use other models or even implement your own.
    let symbols = vec![23i32, -15, 78, 43, -69];
    let quantizer = DefaultLeakyQuantizer::new(-100..=100);
    let means = vec![35.2f64, -1.7, 30.1, 71.2, -75.1];
    let stds = vec![10.1f64, 25.3, 23.8, 35.4, 3.9];
    let models = means.iter().zip(&stds).map(|(&mean, &std)| {
        quantizer.quantize(probability::distribution::Gaussian::new(mean, std))
    });

    // Encode symbols (in *reverse* order, because ANS Coding operates as a stack).
    coder
        .encode_symbols_reverse(symbols.iter().zip(models.clone()))
        .unwrap();

    // Obtain temporary shared access to the compressed bit string. If you want ownership of the
    // compressed bit string, call `.into_compressed()` instead of `.get_compressed()`.
    println!(
        "Encoded into {} bits: {:?}",
        coder.num_bits(),
        &*coder.get_compressed().unwrap()
    );

    // Decode the symbols and verify correctness.
    let reconstructed = coder
        .decode_symbols(models)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert_eq!(reconstructed, symbols);
}

// pub fn example() {
//     println!("Kompakt!");

//     let encoded_symbols = vec![0x421C_7EC3, 0x000B_8ED1];
//     let symbols = vec![23i32, -15, 78, 43, -69]; // [23i32, -15, 78, 43, -69]

//     // assert_eq!(encode_sample_data(), [0x421C_7EC3, 0x000B_8ED1]);
//     if encode_sample_data() == encoded_symbols {
//         println!("Encoded!");
//     }

//     // assert_eq!(decode_sample_data(encoded_symbols), decoded_symbols);
//     if decode_sample_data(encoded_symbols) == symbols {
//         println!("Decoded!");
//     }

//     // run basic example
//     basic_example()
// }

// https://en.wikipedia.org/wiki/PAQ
// https://docs.rs/constriction/0.3.0/constriction/
