#!/bin/bash

cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out xml
