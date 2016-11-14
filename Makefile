#   -------------------------------------------------------------
#   Servers log microservice
#
#   Collects servers log entries and publish them as JSON
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#   Project:        Nasqueron
#   Created:        2016-11-14
#   License:        BSD-2-Clause
#   -------------------------------------------------------------

#   -------------------------------------------------------------
#   Main targets
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

all: build

build: target/release/serverslog

test:
	RUST_TEST_THREADS=1 cargo test

clean:
	rm -rf target/*/serverslog

clean-all:
	cargo clean

#   -------------------------------------------------------------
#   Build
#   - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

target/release/serverslog:
	cargo build --release
