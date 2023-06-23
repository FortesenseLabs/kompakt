## Kompakt Archiver

An Improved General-Pupose Lossless Compression Data Compression/Decompression Library 

**Target: [1GB == 1MB]**

Kompakt aims for minimal compression ratio, runtime memory requirements and some speed if possible by using multi-threaded PAQ-based lossless compression archivers that reduces data by approximately 1024 percent.

**Hash-based compression:** If a hash can be reversed then data can be compressed into the tiniest pieces, i.e hashing data before compression can help reduce its size by a thousand fold.

**Neural Network-based compression:** Using Deep learning by harnessing Neural Network-based modelling combined with arithmetic coding; For this the models are first trained, and the trained models shipped along with the compression/decompression program i.e each model has a unique number (hashed and encoded) and that number is what identifies the model that was used to compress that data so that the data can be decompressed by the appropriate model(Referenced from DZip-torch). 

**Dictionary-based compression:** Group data into 4 hexadecimal form, then generate a dictionary based on it. i.e 16 Permutation 4

**Questions:**

- How to reverse a hash data to it's original form?
-

##### **Programming Languages under consideration to use:**

- Rust
- Golang



##### References:

- https://en.wikipedia.org/wiki/PAQ
- http://www.wu.ece.ufl.edu/links/dataRate/DataMeasurementChart.html



**#################################################**

**Some trashes in my head**

Two-way hash function + pointers = ? [

**Compression:**

```
Given a dataset,
```



```
split the dataset into N chunks, where N is any interger from 0 to length(N)/2,
```


```
Hash each chunk accordingly while append a number/timestamp/pointer to the next hash,
```


```
<Build a dictionary with the hashes>,
```


```
Hashes that appear in more than one section/half should be replaced with a unique ID which references similar hashes
```


**Decompression:**

```
Given a compressed dataset,
```


```
Replace all unique IDs with their reference hashes,
```


```
```


]


**#################################################**

To build zopfli, compile all .c source files under src/zopfli to a single binary with C, and link to the standard C math library, e.g.: gcc src/zopfli/*.c -O2 -W -Wall -Wextra -Wno-unused-function -ansi -pedantic -lm -o zopfli

**#####################References############################**
- https://en.wikipedia.org/wiki/Hash_function
- https://en.wikipedia.org/wiki/Perfect_hash_function
- https://en.wikipedia.org/wiki/PAQ
- http://burtleburtle.net/bob/hash/perfect.html
- https://neuralcompression.github.io/
- https://stackoverflow.com/questions/25436312/gitignore-not-working

