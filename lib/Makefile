# Used to build & test rust-wasm package

JS_DIR=../client
OUT_DIR=$(JS_DIR)/src/crate-build
TARGET=web

# Javascript package manager
JS_PACMAN=npm

# Var that filters out args of makefile
args = `arg="$(filter-out $@,$(MAKECMDGOALS))" && echo $${arg:-${1}}`

all:
	wasm-pack build --out-dir $(OUT_DIR) --target $(TARGET)

tests:
	cargo test -- --nocapture

# Used to pass arguments for testing
# Format is 
# make test {{test name}}
# ex. 
# make test mat4
# all tests are inside src/tests.rs
%:
	@:

test:
	@cargo test tests::$(call args, no_test_defined) -- --nocapture


web:
	cd $(JS_DIR) && BROWSER=NONE $(JS_PACMAN) start

web-browser:
	cd $(JS_DIR) && $(JS_PACMAN) start

build:
	cd $(JS_DIR) && yarn build
