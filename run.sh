#!/bin/bash
cargo b --release 

sudo setcap CAP_NET_ADMIN=eip target/release/tcp

pid=$!
target/release/tcp
trap "kill $pid" INT TERM
wait $pid

