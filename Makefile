PACKAGES = .rust_bin clojure crypto json psql scripts workspace
RUST_DIRECTORY = $(CURDIR)/rust
RUST_PACKAGES = $(shell find $(RUST_DIRECTORY) -mindepth 1 -maxdepth 1 -type d -print)
RUST_BIN_DIRECTORY = $(CURDIR)/.rust_bin

.PHONY: install
install: ensure_local_bin_directory build_rust
	for package in $(PACKAGES); do \
		stow --no-folding --target "$(HOME)/.local" "$$package"; \
	done

.PHONY: uninstall
uninstall:
	for package in $(PACKAGES); do \
		stow --no-folding --delete --target "$(HOME)/.local" "$$package"; \
	done

.PHONY: ensure_local_bin_directory
ensure_local_bin_directory:
	mkdir -p "$(HOME)/.local"

.PHONY: build_rust
build_rust: ensure_rust_bin_directory
	for dir in $(RUST_PACKAGES); do \
		bin=$$(basename $$dir); \
		cd "$$dir" && cargo build --release; \
		cp "$$dir/target/release/$$bin" "$(RUST_BIN_DIRECTORY)/bin/"; \
	done

.PHONY: ensure_rust_bin_directory
ensure_rust_bin_directory:
	mkdir -p "$(RUST_BIN_DIRECTORY)/bin"
