#!/usr/bin/env bash
# Regenerate apollo3-pac from the SVD via chiptool + form. Run from this dir.
set -euxo pipefail

if ! command -v chiptool >/dev/null; then
    echo "chiptool not found. Install:"
    echo "    cargo install --git https://github.com/embassy-rs/chiptool --locked"
    exit 1
fi
if ! command -v form >/dev/null; then
    echo "form not found. Install:"
    echo "    cargo install form"
    exit 1
fi

SVD="svd/apollo3.svd"
TRANSFORM="svd/ambiq.yaml"

if [ ! -f "$SVD" ]; then
    echo "SVD file not found at $SVD!"
    echo "Due to licensing, the original Ambiq Apollo3 SVD is not distributed with this crate."
    echo "Please download the SVD from the Ambiq SDK and place it at 'svd/apollo3.svd'."
    exit 1
fi

# Wipe previous generated source so we don't keep stale files from removed modules.
rm -rf src

# chiptool produces lib.rs + device.x in cwd.
chiptool generate --svd "$SVD" --transform "$TRANSFORM" --output .

# Format before form splits it 
rustfmt --edition 2021 lib.rs

# Split per-peripheral. form writes src/lib.rs (the entry) plus src/<peri>.rs
# and src/<peri>/{regs,vals}.rs for each top-level pub mod.
form -i lib.rs -o src

# form copies the entry into src/lib.rs; the original monolith is now stale.
rm lib.rs

# Format the split output.
find src -name '*.rs' -exec rustfmt --edition 2021 {} +

echo "apollo3-pac regenerated."
