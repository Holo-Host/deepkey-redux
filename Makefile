#
# Test and build DeepKey Project
#
# This Makefile is primarily instructional; you can simply enter the Nix environment for
# holochain-rust development (supplied by holo=nixpkgs; see pkgs.nix) via `nix-shell` and run `hc
# test` directly, or build a target directly (see default.nix), eg. `nix-build -A DeepKey`.
#
SHELL		= bash
DNANAME		= DeepKey
DNA		= dist/$(DNANAME).dna.json
# External targets; Uses a nix-shell environment to obtain Holochain runtimes, run tests, etc.
.PHONY: all
all: nix-test

# nix-test, nix-install, ...
#
# Provides a nix-shell environment, and runs the desired Makefile target.  It is recommended that
# you add `substituters = ...` and `trusted-public-keys = ...` to your nix.conf (see README.md), to
# take advantage of cached Nix and Holo build assets.
nix-%:
	nix-shell --pure --run "make $*"

# Internal targets; require a Nix environment in order to be deterministic.
# - Uses the version of `hc`, `holochain` on the system PATH.
# - Normally called from within a Nix environment, eg. run `nix-shell`
.PHONY:		rebuild install build
rebuild:	clean build

install:	build

build:		$(DNA)

# Build the DNA; Specifying a custom --output requires the path to exist
# However, if the name of the directory within which `hc` is run matches the
# DNA's name, then this name is used by default, and the output directory is
# created automatically.
$(DNA):
	hc package

.PHONY: test test-all test-unit test-e2e test-sim2h test-node
test-all:	test test-stress

test:		test-unit test-e2e

test-unit:
	RUST_BACKTRACE=1 cargo test \
	    -- --nocapture

test-e2e:	$(DNA) test-sim2h test-node
	@echo "Starting Scenario tests in $$(pwd)..."; \
	    RUST_BACKTRACE=1 hc test \
	#        | test/node_modules/faucet/bin/cmd.js

test-node:
	@echo "Setting up Scenario/Stress test Javascript..."; \
	    cd test && npm install

.PHONY: doc-all
doc-all: $(addsuffix .html, $(basename $(wildcard doc/*.org)))

doc/%.html: doc/%.org
	emacs $< --batch -f org-html-export-to-html --kill

# Generic targets; does not require a Nix environment
.PHONY: clean
clean:
	rm -rf \
	    dist \
	    test/node_modules \
	    .cargo \
	    target
