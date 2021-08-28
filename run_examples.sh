#!/bin/bash
set -e

for d in examples/*
do
    example=$(echo $d | sed 's/examples\/\(.*\).rs/\1/')
    echo "Running example $example..."
    cargo run --example $example
done

