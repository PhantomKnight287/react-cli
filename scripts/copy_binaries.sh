#!/bin/bash
for bin in *.tar.gz; do
    tar -xf $bin/$bin
    mv react ../packages/$(basename "$bin" .tar.gz)
done

for bin in *.zip; do
    tar -xf $bin/$bin
    mv react.exe ../packages/$(basename "$bin" .zip)
done
