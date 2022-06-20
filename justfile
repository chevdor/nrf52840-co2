# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

setup:
	rustup default stable
	cargo install probe-run
	cargo install cargo-generate
	cargo install flip-link

show_probes:
	cargo flash --list-probes

run_rtic:
	cargo run --bin rtic -- --probe $VID:$PID:$SERIAL
