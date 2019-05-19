#!/bin/sh
function build() {
  if ~/.cargo/bin/cargo build --release
  then
    mkdir -p build
    rm -rf build/*
    cp target/release/lancelot_life build/server
    cp -r static build/
    echo "Build success !"
  else
    echo "build.sh: Build Failed with code $?"
    exit 1
  fi
}
