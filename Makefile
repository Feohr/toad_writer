crates=$(wildcard crates/*)

all: $(crates)

$(crates): test
	cd $@ && cargo test

test:
	cargo test
