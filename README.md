# buildsa 
This program is for the first half part of the course CMSC701: Computational Genomics Programming Assignment1.

This program will read in a “genome” (in FASTA) format, build the suffix array on this reference, and write the string and suffix array to a binary file. Additionally (when an extra option is provided) it will build a secondary index to allow for the improved search heuristic we discussed in class. When invoked with this extra option, it will also write this data structure to the serialized file.

## Input

The input consists of one optional parameter, as well as 2 required arguments, given in this order:

- `--preftab` <k> - if the option `--preftab` is passed to the `buildsa` executable (with the parameter `k`), then a prefix table will be built atop the suffix array, capable of jumping to the suffix array interval corresponding to any prefix of length `k`.

- `reference` - the path to a FASTA format file containing the reference of which you will build the suffix array. Note: The FASTA format allows a record to be split over multiple input lines, make sure your program is parsing the reference input correctly.

- `output` - the program will write a single binary output file to a file with this name, that contains a serialized version of the input string and the suffix array.

## Output

- The program will output a file with the name given by the output argument above. This should be a binary file holding everything necessary to perform query using your suffix array. Specifically, it should include the input string itself (probably with the sentinel `$` appended) and it should also include an encoding of the entries of the suffix array. Aditionally, if your program created a prefix table (was given the `--preftab` option), then this file will also contain the serialzation of the prefix table.

## Example

To run this programon Linux, download the whole `buildsa` folder, go to the `buildsa` directory and make sure the parameter given by this order: `reference`, `preftab`, `output`. If you don't want to construct the prefix loop up table, just let the `preftab` parameter = 0.

Here is an example:

```{bash}
$ cargo run ecoli.fa 12 output_ecoli_12mer.bin
```