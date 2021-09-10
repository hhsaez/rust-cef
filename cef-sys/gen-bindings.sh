#!/bin/bash

unameOut="$(uname -s)"
case "${unameOut}" in
    Linux*)     outfile=src/bindings_linux.rs;;
    Darwin*)    outfile=src/bindings_mac.rs;;
    CYGWIN*)    outfile=src/bindings_msvc.rs;;
    MINGW*)     outfile=src/bindings_msvc.rs;;
    *)          echo "Unknown platform"; exit 1
esac

echo "Generating to: ${outfile}"

bindgen wrapper.h -o $outfile \
    --rust-target nightly \
    --default-enum-style=rust_non_exhaustive \
    --whitelist-type cef_.* \
    --whitelist-function cef_.* \
    --bitfield-enum .*_mask_t \
    -- -I $CEF_ROOT
