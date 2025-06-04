# Build the Rust binary
build:
	cargo build --release

# Run using cargo
run:
	cargo run -- --input your-input.gml --output your-output-dir

# Build Docker image
docker-build:
	docker build -t rustcitygml2obj .

# Run in Docker (modify input/output path to absolute ones)
docker-run:
	docker run --rm -v $(PWD):/data rustcitygml2obj \
		--input /data/your-input.gml --output /data/output

# Test (placeholder, add actual tests)
test:
	cargo test
