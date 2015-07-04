#!/bin/bash

cd /tmp/rust-sinoparserd

if [ -z "$1" ]; then
    cargo run
else
    exec "$@"
fi


