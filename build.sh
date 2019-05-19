#!/bin/sh
mkdir -p build
rm -rf build/*
cp target/release/lancelot_life build/server
cp -r static build/