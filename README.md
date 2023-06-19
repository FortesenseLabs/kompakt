# Kompakt

Kompakt is a general-purpose data compression library that aims to achieve a compression ratio of approximately 1024:1. The library uses multi-threaded PAQ-based lossless compression archivers to reduce data by approximately 80% - 100%.

For a compression ratio of 1024:1 we have 1GB - 1MB. This means that the compressed data will be approximately 1/1024th of the original size. For example, if you have 1GB (1024 megabytes) of data, after compression with a compression ratio of 1024:1, the compressed data will be approximately 1MB in size.

## Some Ideas

**Hash-based compression:** If a hash can be reversed then data can be compressed into the tiniest pieces, i.e hashing data before compression can help reduce its size by a thousand fold.

**Neural Network-based compression:** Using Deep learning by harnessing Neural Network-based modelling combined with arithmetic coding; For this the models are first trained, and the trained models shipped along with the compression/decompression program i.e each model has a unique number (hashed and encoded) and that number is what identifies the model that was used to compress that data so that the data can be decompressed by the appropriate model(Referenced from DZip-torch).

**Dictionary-based compression:** Group data into 4 hexadecimal form, then generate a dictionary based on it. i.e 16 Permutation 4

**Questions:**

- How to reverse a hash data to it's original form?
- Would the mixture of encoding algorithms such as shannon, huffman, and lempel-ziv give a middle-out compression algorithm?

## Resources

https://www.notion.so/Data-compression-c29e688a2e3442d9af3c015a2a7d71be

https://github.com/bamler-lab/constriction

compression as a service [Google Search]

https://github.com/ABuarque/simple-compression-service

https://nebula.esa.int/content/data-compression-service

https://en.wikipedia.org/wiki/Shannon%E2%80%93Fano_coding

https://en.wikipedia.org/wiki/Huffman_coding

https://en.wikipedia.org/wiki/Lempel%E2%80%93Ziv%E2%80%93Welch

**Encoding Algorithms:**

- Shannon
- Huffman
- Lempel-ziv
