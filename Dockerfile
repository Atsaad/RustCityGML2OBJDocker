# Stage 1: Build the Rust binary
FROM rustlang/rust:nightly AS builder

# Set workdir
WORKDIR /usr/src/app

# Copy source code
COPY . .

# Build release binary
RUN cargo build --release

# Stage 2: Create minimal image
FROM debian:bookworm

# Install lib dependencies if needed (e.g. OpenSSL)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /usr/src/app/target/release/RustCityGML2OBJ /usr/local/bin/rustcitygml2obj

# Set the entrypoint
ENTRYPOINT ["rustcitygml2obj"]
