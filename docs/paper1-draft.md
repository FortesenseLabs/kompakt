## Data compression using deterministic random number generators and deep  probabilistic models (draft)

**Keywords:** deterministic, random number, generators, PRNGs*

Partitioning: That is, if we are given a set of random numbers called A, we first divide A into K subsets with each subset having a total size of N.

For each subset, find a seed called S such that when S is given to a deterministic random number generator, the output of the generator is equal to the subset.

![Data Compression](../images/1.jpg)

1st image: On the right side trying to calculate the total combination of hexadecimal number system 

Conversion of Binary to Decimal technique 


**For the Data Description Language Approach:**

The data would be divided into two sections, as stated below

1. Metadata section: A set of data that describes and gives information about other data. Holds things like the type of data or data extension (.jpg) 
2. Data section: Holds the generator functions 

f(x) ⇒ y, where y is z(y) ⇒ x | f() and z() are encoding and decoding functions respectively. For generating the data functions in the data section, two approaches are been considered namely: Random deterministic approach; and Using Machine Learning/Deep Learning to model the relationship between the data and it's selected generator function, and vice versa (i.e using ML/DL to generate the functions and converting this same functions back to the bytes).

![Data DDL](../images/2.jpg)


**Text Summary Model Approach:** 

The ability to summarize text and enlarge the summary to get the original version back. That means context and depth would go a long way.

**Numenta's SDR Like**

[Sparse Distributed Representations](https://www.numenta.com/assets/pdf/biological-and-machine-intelligence/BaMI-SDR.pdf)

[Encoders](https://www.numenta.com/assets/pdf/biological-and-machine-intelligence/BaMI-Encoders.pdf)

![Data SDR](../images/3.jpg)

## Pseudo-Codes:
***Extreme data compression using random numbers*** 

"Random" Number Generators

```typescript
function rebuildArray (seed: UInt32, modulus: Int, length: Int):[Int] { 
    var generatedArray = [Int] (count: length, repeatedValue: 0)

    srandom(seed)

    for i in 0..length {

    }

    let nextInt = random() modulus generatedArray[i] = nextInt

    return generatedArray
}
```

**Pseudo-Code to find seed:**
```typescript
let v = data

let vl= len(data)

var bestDist = 2^vl 1 //ALL 0xFFS

var bestSeed=0

var seed = 8

for (i=0; i< 2^vl; i++) {

seed = i

let testBits = randomBitString(1, 255, vl)

let diff distance(testBits, data)

if (diff bestDist){

bestDist = diff

bestSeed = seed

}

return bestSeed

```

**Results:** 

```typescript
rebuildArray(20160401, modulus: 1000, Length: 30)

//[802, 309, 571, 807, 639, 637, 89, 894, 953, 325, 92, 649, 509, 377, 693, 248, 937, 83, 526, 59, 628, 602, 322, 786, 202, 212, 586, 739, 518, 89]
```


## Versions 
A picture and video version. The video version can be used it streaming sites, it would still work well under poor network constraints (think of the media player as an interpreter and the video as a compiled code, imagine been able to stream a 160mins worth of 360p video(s) with just a max of 10MB) 

## Use cases
- Decentralized networks like Tor and Bitcoin 
- web browsers
- streaming (audio or video)
- etc


## References

- [Prof. Balmer's Constriction](https://github.com/bamler-lab/constriction)
- https://m.youtube.com/watch?v=KOvoD1upTxM
- https://m.youtube.com/watch?v=kf_p60xSQSs
- https://www.notion.so/seekersoftec/Data-Compression-9690604b72e743d0a4da0bc6eb918df3
- https://github.com/laurauzcategui/fastapi_ml_stablediffusion/



**Note:** some of the key Ideas were borrowed concepts 