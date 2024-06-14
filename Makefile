# Variables
CARGO := cargo
CROSS := cross

# Default target
.PHONY: all
all: build test

# Build rule
.PHONY: build
build:
	@echo "Building for all platforms:"
	@echo "    -> aarch64-unknown-linux-gnu"
	$(CROSS) build --target aarch64-unknown-linux-gnu --release
	@echo "    -> i686-pc-windows-gnu"
	$(CROSS) build --target i686-pc-windows-gnu --release
	@echo "    -> i686-unknown-linux-gnu"
	$(CROSS) build --target i686-unknown-linux-gnu --release
	@# @echo "    -> x86_64-apple-darwin"
	@# $(CROSS) build --target x86_64-apple-darwin --release
	@echo "    -> x86_64-pc-windows-gnu"
	$(CROSS) build --target x86_64-pc-windows-gnu --release
	@echo "    -> x86_64-unknown-linux-gnu"
	$(CROSS) build --target x86_64-unknown-linux-gnu --release

# Test rule
.PHONY: test
test:
	@echo "Testing on all platforms:"
	@echo "    -> Linux"
	$(CROSS) test --target i686-unknown-linux-gnu --release
	@echo "    -> Windows"
	$(CROSS) test --target i686-pc-windows-gnu --release
	@# @echo "    -> MacOS"
	@# $(CROSS) test --target x86_64-apple-darwin --release

.PHONY: install_cross
install_cross:
	cargo install cross --git https://github.com/cross-rs/cross

# Clean rule
.PHONY: clean
clean:
	$(CARGO) clean
