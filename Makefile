.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run example/calculator.lingua 
	echo "Running bin/main file"
	bin/main