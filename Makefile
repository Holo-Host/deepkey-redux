.PHONY: all test test-unit test-e2e install clean

all: test

#
# test -- Use nix-shell environment to obtain Holochain runtime, run tests
#
test:
	nix-shell --run hf-test

# test-unit:
# 	nix-shell --run hf-test-unit
#
# test-e2e:
# 	nix-shell --run hf-test-e2e
#
# install:
# 	nix-shell --run hf-install
#
# clean:
# 	rm -rf dist test/node_modules .cargo # Only cleans up holofuel artifacts
#
#
