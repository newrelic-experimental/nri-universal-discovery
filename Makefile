setup:
		@echo "===================================================================="
		@echo "Setting up..."
		@echo "===================================================================="
		@echo "Add Windows targets"
		@echo "===================================================================="
		rustup target add x86_64-pc-windows-gnu
		@echo "===================================================================="
		@echo "Add Linux targets"
		@echo "===================================================================="
		rustup target add x86_64-unknown-linux-musl

clean-target:
		@echo "===================================================================="
		@echo "Cleaning target directory..."
		@echo "===================================================================="
		rm -rf target/*


build:
		@echo "===================================================================="
		@echo "Building..."
		@echo "===================================================================="
		cargo build

build-windows: clean-target
		@echo "===================================================================="
		@echo "Building windows release..."
		@echo "===================================================================="
		cargo build --release --target=x86_64-pc-windows-gnu

build-linux: clean-target
		@echo "===================================================================="
		@echo "Building linux release... x86_64-unknown-linux-musl"
		@echo "===================================================================="
		cargo build --release --target=x86_64-unknown-linux-musl

build-all: build-linux build-windows

test:
		@echo "===================================================================="
		@echo "Testing..."
		@echo "===================================================================="
		cargo test

package-linux: build-linux
		@echo "===================================================================="
		@echo "Packaging linux release..."
		@echo "===================================================================="
		./build/package.sh linux-x86_64 x86_64-unknown-linux-musl
	