.PHONY: install
install: scripts

.PHONY: scripts
scripts:
	stow --no-folding --target $(HOME) scripts

