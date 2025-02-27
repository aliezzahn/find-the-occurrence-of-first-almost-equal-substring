# Find the Occurrence of the First Almost Equal Substring

A Rust library to find the occurrence of the first almost equal substring in a given string.

![CI](https://github.com/aliezzahn/find-the-occurrence-of-first-almost-equal-substring/actions/workflows/ci.yml/badge.svg)
![CD](https://github.com/aliezzahn/find-the-occurrence-of-first-almost-equal-substring/actions/workflows/cd.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

This library provides a function `min_starting_index` that searches for the first occurrence of a substring in a given string that is "almost equal" to a specified pattern. The term "almost equal" means that the substring and the pattern can differ by at most one character.

## Usage

Add the library to your `Cargo.toml`:

```toml
[dependencies]
find-the-occurrence-of-first-almost-equal-substring = "0.1"
```

Then, use the library in your Rust code:

```rust
use find_the_occurrence_of_first_almost_equal_substring::min_starting_index;

fn main() {
    let s = "hello";
    let pattern = "helo";
    let index = min_starting_index(s, pattern);
    println!("Starting index: {}", index); // Output: Starting index: 0
}
```

## Examples

- Exact match:

  ```rust
  assert_eq!(min_starting_index("hello", "hello"), 0);
  ```

- Almost equal match:

  ```rust
  assert_eq!(min_starting_index("hello", "helo"), 0);
  ```

- No match:
  ```rust
  assert_eq!(min_starting_index("hello", "world"), -1);
  ```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/aliezzahn/find-the-occurrence-of-first-almost-equal-substring).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

- Email: aliezzahn@gmail.com
- GitHub: [aliezzahn](https://github.com/aliezzahn)
