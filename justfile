
# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

setup:
	rustup default stable
	cargo install probe-run
	cargo install cargo-generate
	cargo install flip-link

