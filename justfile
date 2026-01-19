# Run all verification tasks
all:
    task validate-all

# Check all model workspaces
check:
    task check-all

# Format all model workspaces
fmt:
    task fmt-all

# Lint all model workspaces
lint:
    task lint-all

# Run all tests
test:
    task test-all

# Clean all target directories
clean:
    task clean-all

# Generate the directory structure using the Rust tool
gen:
    cargo run --release
