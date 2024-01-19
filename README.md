![Rust CI](https://github.com/TheRealPad/rubiks-cube-solver/actions/workflows/ci.yml/badge.svg)
![Docker deploy](https://github.com/TheRealPad/rubiks-cube-solver/actions/workflows/deploy.yml/badge.svg)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

# rubiks-cube-solver

Program to solve 3x3 Rubik's Cube with Fridrich method.

[Methode (french)](documentation/fridrich.pdf)

## How to run

### Install dependencies

[Rust](https://www.rust-lang.org/tools/install)

[Docker](https://docs.docker.com/get-docker/)

### Run

```bash
cargo run
# or
docker-compose up # --build to start fresh
```

### Test

```bash
cargo test
```

### Documentation

You can generate documentation by running ```cargo doc```