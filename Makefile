.PHONY: install
install: scripts

.PHONY: scripts
scripts:
	stow --no-folding --target $(HOME) scripts

.PHONY: uninstall
uninstall:
	stow --no-folding --delete --target $(HOME) scripts
