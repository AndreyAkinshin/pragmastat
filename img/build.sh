#!/bin/bash

set -ux

cd "$(dirname "$0")" || exit 1

Rscript generate-images.R