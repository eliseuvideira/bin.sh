PACKAGES = crypto json psql scripts workspace

.PHONY: install
install: ensure_local
	for package in $(PACKAGES); do \
		stow --no-folding --target "$(HOME)/.local" "$$package"; \
	done

.PHONY: uninstall
uninstall:
	for package in $(PACKAGES); do \
		stow --no-folding --delete --target "$(HOME)/.local" "$$package"; \
	done

.PHONY: ensure_local
ensure_local:
	mkdir -p "$(HOME)/.local"
