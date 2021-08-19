#!/bin/bash

cargo build --release
while getopts n:a:o: flag
do
    case "${flag}" in
        n) name=${OPTARG};;
        a) arguments=${OPTARG};;
        o) outputs=${OPTARG};;
    esac
done
echo "Ugen Name: $name";
echo "UGen Arguments: $arguments";
echo "UGen Outputs: $outputs";

cd genscwrapper
cargo run -- --name $name --outputs $outputs -a $arguments
cd ../sc
make ugen UGEN=$name
cd ..
