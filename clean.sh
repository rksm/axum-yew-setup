#!/usr/bin/env bash

find . \( -name target -o -name target-trunk -o -name dist \) -type d | xargs rm -rf
