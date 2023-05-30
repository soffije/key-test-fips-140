# FIPS-140 Tests

The provided code implements a series of tests to assess the randomness of a sequence of 20,000 bits. Here's an overview of the code:

1. The code imports the `rand::Rng` module, which is used for generating random numbers.

2. The `generate_random_sequence` function generates a random sequence of 2,500 bytes (20,000 bits). It uses the `rand::thread_rng()` function to create a random number generator and iteratively generates random bytes, appending them to a vector.

3. The `monobit_test` function performs a monobit test on the input sequence. It counts the number of ones and zeros in the sequence and checks if they fall within a specific range. The expected range for both ones and zeros is between 9,654 and 10,345 (inclusive). If the counts are within the expected range, the function returns `true`, indicating that the sequence passes the test; otherwise, it returns `false`.

4. The `series_test` function checks the maximum length of consecutive series of ones and zeros in the input sequence. It iterates over the sequence, maintaining counts for the current series length and the maximum series length for both ones and zeros. If any series exceeds a length of 36, the function returns `false`; otherwise, it returns `true`.

5. The `poker_test` function applies the Poker test to the input sequence. It divides the sequence into blocks of length 4 and counts the occurrences of each possible block configuration (16 possibilities). It then calculates the test statistic X^2 based on the counts and compares it to a predetermined range. If X^2 falls within the range of 1 to 57 (inclusive), the function returns `true`; otherwise, it returns `false`.

6. The `calculate_block_index` function calculates the index of a poker block based on its byte representation. It iterates over the bits of the block and constructs the index by shifting left and performing bitwise OR operations.

7. The `series_length_test` function checks the lengths of series of ones and zeros in the input sequence. It iterates over the sequence and updates counters for the lengths of consecutive series. If the counts for any series length do not fall within the specified ranges, the function returns `false`; otherwise, it returns `true`.

8. The `main` function generates a random sequence using `generate_random_sequence` and then applies the four tests to determine if the sequence is random. If all tests pass, it prints "The sequence of 20,000 bits is random"; otherwise, it prints "The sequence of 20,000 bits is not random."

Overall, the code evaluates randomness based on criteria such as the distribution of ones and zeros, the lengths of consecutive series, and the statistical properties of poker blocks. It provides a basic assessment of the randomness of the input sequence.
