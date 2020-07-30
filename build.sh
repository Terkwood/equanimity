#!/bin/bash

wasm-pack build --target web --out-name wasm --out-dir ./dist
cp static/*.html dist/.
cp static/*.css dist/.
