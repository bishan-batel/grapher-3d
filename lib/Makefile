# Used to build & test rust-wasm package

OUT_DIR="../client/src/crate-build"
TARGET="web"

# Javascript package manager
JS_PACMAN=yarn

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
	cd ..
	BROWSER=NONE $(JS_PACMAN) start

web-browser:
	cd ..
	$(JS_PACMAN) start
