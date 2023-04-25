# Find the missing number

This repository is a Rust implementation to find the first missing number in a collection of randomly generated numbers.

In this implementation,
- random numbers are in the range of `0..u16::MAX` (`0`..`65_535`),
- the `Vec` holding the random numbers has a capacity of `64_000`, meaning, that there will always be `1536` numbers missing in the collection.

The random numbers and initialization of the `Vec` holding them are used to setup the problem only. In other words, you can think of this as just _input_ to the problem of finding the missing number.

## Constraints

To make the problem challenging, the following constraints exist:
- You cannot manipulate the `Vec` containing the random numbers. So, sorting the `Vec` and then iterating though it to find the first missing number is not allowed.
- You can only use 8K of memory (data) in the solution.

## Solution

The solution in this repository uses bits to keep track of the missing numbers. Since we know the range of valid numbers is between `0`..`65_535` (64K), it is possible to represent all the numbers (missing included) using only 8K of data. For example, if looking at just the firsr 8 bytes of a sample run, you could see the following output.

```
1023: 11111111 11111111 11111111 11111111 11111111 11111111 11110111 11111111
                                                                ^
     | byte 7 | byte 6 | byte 5 | byte 4 | byte 3 | byte 2 | byte 1 | byte 0 |
```

The above represents the first 64 numbers in the collection using a bit array, where number 11 (see byte 1) is missing.

## Running the solution

To run the solution, simply run `cargo run`. The output will print the entire bit array representing all random numbers in the Vec, which has a capacity of 64K.

Next, the output will print the _first_ and _last_ missing number in the bit array, as well as, the _total_ number of missing numbers. For example,

```
1018: 11111111 11111111 11111111 01111111 11111111 11111111 11111111 11111111 
1019: 11111111 11111111 11111111 01011111 11111111 11111111 01111111 11111111 
1020: 11111111 11111111 11111111 11111111 11111111 11111111 11111011 11111111 
1021: 11111111 11111111 11111111 11111111 11111111 11111111 11111111 11111111 
1022: 11111111 11111111 11111111 11111111 11111111 11111111 11111111 11110111 
1023: 11111111 11111111 11111111 11111111 11111111 11111111 11110111 11111111 
First missing number: 11
Last missing number: 65467
Total missing numbers: 1536
```






