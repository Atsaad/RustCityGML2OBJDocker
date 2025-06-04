## 🏧 RustCityGML2OBJ (Dockerized)

**RustCityGML2OBJ** is a Rust-based tool for converting CityGML files to OBJ format. This documentation outlines how to use, build, and run the application via Docker and includes the associated automation via GitHub Actions.

---

### 📁 Project Structure

```
RustCityGML2OBJ/
├── src/
│   ├── conversion_functions.rs
│   ├── geometry_functions.rs
│   ├── translation_module.rs
│   ├── write_functions.rs
├── input/                      # Place your .gml / .xml files here
├── output/                     # Output OBJ and JSON files appear here
├── Cargo.toml
├── Dockerfile
├── Makefile
├── .github/
│   └── workflows/
│       └── docker.yaml
└── README.md
```

---

### 🐋 Dockerfile Overview

A two-stage Dockerfile:

- **Stage 1**: Compiles the Rust binary using the official Rust image.
- **Stage 2**: Creates a minimal Debian-based runtime container with the binary.

```Dockerfile
# Stage 1: Build the Rust binary
FROM rustlang/rust:nightly AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Stage 2: Minimal runtime image
FROM debian:bookworm
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/RustCityGML2OBJ /usr/local/bin/rustcitygml2obj
ENTRYPOINT ["rustcitygml2obj"]
```

---

### 🛠️ Makefile Commands

```make
# Build binary
build:
	cargo build --release

# Run locally
run:
	cargo run -- --input input --output output

# Docker: Build image
docker-build:
	docker build -t rustcitygml2obj .

# Docker: Run container (PWD is auto-mounted)
docker-run:
	docker run --rm -v $(PWD):/data rustcitygml2obj \
		--input /data/input --output /data/output

# Run tests
test:
	cargo test
```

---

### ⚙️ GitHub Actions CI

`.github/workflows/docker.yaml` builds the project on every push/pull request to `main`, tests it, and builds the Docker image.

Key steps:

- Checkout code  
- Set up Rust  
- Cache Cargo dependencies  
- Build with `cargo build --release`  
- Run `--help` test on compiled binary  
- Build Docker image  

---

### 🧪 Running the Tool in Docker (PowerShell)

If you're using **PowerShell**, run it like this:

```powershell
docker run --rm -v "$(Get-Location):/data" rustcitygml2obj `
  --input /data/input `
  --output /data/output `
  --tbw `
  --add-json
```

Ensure `input/` contains `.gml` or `.xml` files, and `output/` exists or will be created.

---

### 🔍 CLI Options

```bash
Usage: rustcitygml2obj [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>      Input file path (directory with GML/XML files)
  -o, --output <OUTPUT>    Output directory
      --tbw                Translate buildings into local CRS
      --add-json           Export metadata as JSON
      --add-bb             Include bounding boxes in OBJ
      --import-bb          Import a predefined bounding box
  -h, --help               Show help
  -V, --version            Show version
```

---

### 🗜️ Clean Build (Optional)

```bash
docker system prune -f   # Clean unused Docker data
cargo clean              # Clean Rust build artifacts
```

---

### ♻️ Example Workflow

```bash
# Build Docker image
make docker-build

# Run conversion inside Docker
make docker-run
```

Or directly:

```bash
docker run --rm -v "$(PWD):/data" rustcitygml2obj `
  --input /data/input `
  --output /data/output `
  --tbw --add-json
```

---

### 📌 Notes

- Ensure that `.gml` or `.xml` files exist in `input/` directory.
- Output files (`.obj`, `.json`) are saved to `output/`.
- Output metadata is optional (`--add-json`).
- The tool leverages multi-threading with `rayon` for performance.

