.PHONY: all
all: check
	$(MAKE) --no-print-directory test

.PHONY: test
test:
	# Single thread only so that the results are reported in expected order,
	# otherwise they can run in whatever order with N threads.
	cargo test -- --test-threads 1

.PHONY: check
check:
	cargo check
	cargo clippy
	cargo fmt --check

day_%:
	mkdir -p tests/input/$@
	touch tests/input/$@/input.txt
	touch tests/input/$@/example.txt
	cp day_x.rs src/$@.rs
	echo "day!(_$*, 0, 0, 0, 0);" >> tests/tests.rs
	echo "pub mod day_$*;" >> src/lib.rs
