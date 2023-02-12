.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run example/calculator.calculon

.PHONY: llvm-analyze
llvm-analyze:
	llvm-bcanalyzer --dump bin/main.bc