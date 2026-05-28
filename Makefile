run:
	cargo run

watch:
	watchexec --notify -r -- cargo run

test:
	watchexec --notify -r -- cargo test

doc:
	cargo doc && python3 -m http.server 1989 --directory target/doc
