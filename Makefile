CARGO ?= cargo

.PHONY: all build test clippy fmt snapshot desktop bare clean

all: build test clippy bare

build:
	$(CARGO) build --workspace

test:
	$(CARGO) test --workspace

clippy:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

fmt:
	$(CARGO) fmt --all
	cd targets/bare && $(CARGO) fmt

# Renders the UI states into artifacts/*.png
snapshot:
	$(CARGO) run -p snapshot

# The interactive window
desktop:
	$(CARGO) run -p desktop

# The no_std gate: the same application crate for a bare-metal target
bare:
	cd targets/bare && $(CARGO) build --target riscv64gc-unknown-none-elf

clean:
	$(CARGO) clean
	cd targets/bare && $(CARGO) clean
