#!/bin/bash

cargo build --release
cd sc/
make ugen
make install
