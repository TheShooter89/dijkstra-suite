run:
	cargo run

watch:
	watchexec --notify -r -- cargo run

test:
	watchexec --notify -r -- cargo test
