#!/bin/bash

set -u

cd "$(dirname "$0")" || exit 1

BASENAME=pragmastat

VERSION=$(cat ../manual/version.txt)

if [ "${1:-}" == "--release" ]; then
  FILENAME="$BASENAME-v$VERSION.pdf"
else
  FILENAME="$BASENAME-v$VERSION-draft.pdf"
fi

Rscript -e "rmarkdown::render('$BASENAME.Rmd')"
mv "$BASENAME.pdf" "$FILENAME"

echo -e "Result: \033[32m$FILENAME\033[0m"
