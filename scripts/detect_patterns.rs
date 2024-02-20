// write and implement a algorithm in python that detects patterns in binary data

// This algorithm takes binary data as input (data), along with the desired pattern length (pattern_length). It iterates over the data, extracting patterns of the specified length. If a pattern is not already present in the patterns vector, it is added. The algorithm then returns the detected patterns.

// In the main function, an example binary data sequence and pattern length are provided. The detect_patterns function is called with these inputs, and the resulting patterns are printed.

// You can modify the binary data and pattern length as per your requirements. The algorithm will detect and print all unique patterns of the specified length in the binary data.
fn detect_bin_patterns(data: &[u8], pattern_length: usize) -> Vec<Vec<u8>> {
    let mut patterns: Vec<Vec<u8>> = Vec::new();

    for i in 0..(data.len() - pattern_length + 1) {
        let pattern = &data[i..(i + pattern_length)];

        // Check if the pattern already exists
        let is_duplicate = patterns.iter().any(|p| *p == pattern);
        if !is_duplicate {
            patterns.push(pattern.to_vec());
        }
    }

    patterns
}

fn main() {
    let binary_data: Vec<u8> = vec![0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1];
    let pattern_length = 3;

    let patterns = detect_bin_patterns(&binary_data, pattern_length);

    for pattern in patterns {
        println!("{:?}", pattern);
    }
}


// https://www.researchgate.net/publication/305378724_Local_binary_pattern_network_A_deep_learning_approach_for_face_recognition
// https://www.sciencedirect.com/science/article/pii/S1877050918308986