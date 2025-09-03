TARGET=MSPM0L222X

all: patch generate format

# Patch the SVD file
patch:
	svdtools patch svd/$(TARGET).yaml svd/$(TARGET).patched.svd

# Generate Rust code from the SVD file
generate:
	svd2rust --target=cortex-m --reexport-core-peripherals -i svd/$(TARGET).patched.svd -o .
	rm -r src
	form -i lib.rs -o src/
	rm lib.rs

# Format the generated code
format:
	cargo fmt --all

.PHONY: all patch generate format