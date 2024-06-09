.PHONY: build

all: build

build:
	@killall trunk > /dev/null 2> /dev/null || true
	@env RUSTFLAGS="--cfg=web_sys_unstable_apis" trunk build
