
def detect_bin_patterns(data, pattern_length):
    patterns = []

    for i in range(len(data) - pattern_length + 1):
        pattern = data[i:i + pattern_length]

        # Check if the pattern already exists
        is_duplicate = any(p == pattern for p in patterns)
        if not is_duplicate:
            patterns.append(pattern[:])

    return patterns

def main():
    binary_data = [0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1]
    pattern_length = 2

    patterns = detect_bin_patterns(binary_data, pattern_length)

    for pattern in patterns:
        print(pattern)

if __name__ == "__main__":
    main()

# write and implement a algorithm in python that detects patterns in binary data

# This algorithm takes binary data as input (data), along with the desired pattern length (pattern_length). It iterates over the data, extracting patterns of the specified length. If a pattern is not already present in the patterns vector, it is added. The algorithm then returns the detected patterns.

# In the main function, an example binary data sequence and pattern length are provided. The detect_patterns function is called with these inputs, and the resulting patterns are printed.

# You can modify the binary data and pattern length as per your requirements. The algorithm will detect and print all unique patterns of the specified length in the binary data.


# https://www.researchgate.net/publication/305378724_Local_binary_pattern_network_A_deep_learning_approach_for_face_recognition
# https://www.sciencedirect.com/science/article/pii/S1877050918308986