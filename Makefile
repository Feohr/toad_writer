crates=$(wildcard crate/*)

all: $(crates)

$(crates): test
	cd $@ && cargo test

test:
	cargo test
