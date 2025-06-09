# Ring Arithmetic - HEXL bindinds

## Overview
The Ring Arithmetic Library is a high-performance library for performing arithmetic operations within mathematical rings. It is designed for use in cryptographic applications, modular arithmetic, and abstract algebra. The library leverages Intel's HEXL for optimised modular arithmetic operations.

## Features
- **Efficient Modular Arithmetic**: Addition, subtraction, multiplication, and modular reduction.
- **NTT Support**: Forward and inverse Number Theoretic Transform (NTT) for polynomial arithmetic.
- **Cyclotomic Rings**: Support for arithmetic in cyclotomic rings.
- **Benchmarking**: Integrated benchmarking using Criterion.
- **Customizable Parameters**: Easily configure modulus and ring size.

## Installation

### Prerequisites
- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **C++ Compiler**: A C++17-compatible compiler (e.g., GCC or Clang).
- **CMake**: Version 3.13 or higher.

### Clone the Repository with submodules
```bash
git submodule update --init --recursive
```

## Build and Usage

1. Build the HEXL library
```bash
make hexl
```

2. Build the wrapper
```bash
make wrapper
```

3. Set the library path for the HEXL wrapper:

```bash
export LD_LIBRARY_PATH=./hexl-bindings/hexl/build/hexl/lib:$(pwd)
```

4a. The main program demonstrates basic arithmetic operations in cyclotomic rings:
```bash
cargo run
```


4b. Run the unit tests to verify functionality:
```bash
cargo test
```

4c. Benchmark the performance of arithmetic operations:
```bash
cargo bench
```

## File Structure
 - `src/`: Contains the Rust source code.
 - `hexl-bindings/`: Contains the C++ HEXL wrapper and bindings.
 - `benches/`: Contains benchmarking code.
 - `Makefile`: Build instructions for the HEXL library and wrapper.
