# Makefile for building Rust example binaries

# Find all .rs files in the current directory
SOURCES = $(wildcard ./*/*.rs)
# Create binary names by removing .rs extension
BINARIES = $(SOURCES:.rs=)

# Default target
all: $(BINARIES)

# Rule to build each binary
%: %.rs
	rustc -o $@ $<

# Clean up binaries
clean:
	rm -f $(BINARIES)

.PHONY: all clean
