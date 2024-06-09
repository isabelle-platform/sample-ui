#!/bin/bash
TOP_DIR="$(cd "$(dirname "$(which "$0")")" ; pwd -P)"
cd "$TOP_DIR"

port="$1"

if [ "$port" == "" ] ; then
	port="8081"
fi

killall trunk
# trunk build
RUSTFLAGS=--cfg=web_sys_unstable_apis trunk serve --port "$port"
