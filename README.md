# Math-Lab
This repository contains modular projects for the analysis, simulation and visualization of mathematical problems.

**Math Lab** is a collection of mathematical experiments, algorithms, and number-theoretic explorations implemented in both **Python** and **Rust**.  
The repository aims to provide clean, modular and scalable implementations of classic and modern math problems — from iterative sequences like the *3n+1 problem* to perfect numbers, prime algorithms, and more.  
It is designed as a long-term playground for exploring performance, visualization, and computational mathematics.

---

## Table of Contents
- [Implemented Topics](#implemented-topics)
- [Project Structure](#project-structure)
- [Installation and Setup](#installation-and-setup)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Credits](#credits)
- [Disclaimer](#disclaimer)

---

## Implemented Topics

Initial mathematical problems included in the repository:

- **3n+1 (Collatz Conjecture)**  
  - Python and Rust implementations  
  - Efficient sequence generation  
  - Optional visualization options

Future planned topics:

- Perfect numbers  
- Prime tests and prime generators  
- Numerical sequences  
- Integer factorization experiments  
- Additional number-theory explorations

This list will grow over time as more modules are added.

---

## Project Structure
The repository is organized by mathematical problem.  
Each topic contains implementations in one or more languages:

math-lab/
│
├── 3n+1/
│ ├── python/
│ └── rust/
│
├── perfect-numbers/ (planned)
├── primes/ (planned)
│
├── README.md
└── LICENSE

---

## Installation and Setup
### Python
Install Python dependencies (if present):

```bash
pip install -r requirements.txt
```

### Rust
Ensure Rust and Cargo are installed:
```bash
rustc --version
cargo --version
```

## Usage
### Python
Navigate into any mathematical module and run the Python implementation:

```bash
cd 3n+1/python
python collatz.py
```
### Rust
Build and run a Rust implementation:
```bash
cd 3n+1/rust
cargo run --release
```
---


## Contributing
Contributions, improvements, and discussions are warmly welcome!
If you want to contribute: 
- Fork the repository
- Create a feature branch: ```git checkout -b feature/your-feature```
- Commit your changes: ```git commit -m "Add your feature"```
- Push your branch: ```git push origin feature/your-feature```
- Open a pull request

Feel free to open issues for ideas, questions, or suggestions.

---

## License
This project is licensed under the MIT License.
See the LICENSE file for more details.

---

## Credits
Created and maintained by sp8cky, exploring mathematical computation in Python and Rust.

---

## Disclaimer
This project is provided for educational and experimental purposes.
All computations, visualizations and algorithms are offered as is.
Use at your own risk. Performance, correctness or long-running computations cannot be guaranteed for every numerical scenario.

---
---