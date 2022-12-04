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
