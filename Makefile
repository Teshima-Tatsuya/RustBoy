.PHONY: all clean

test:
	cargo test

reg:
	reg-cli ./tests/actual ./tests/expect ./tests/diff

reg-update:
	reg-cli ./tests/actual ./tests/expect ./tests/diff -U