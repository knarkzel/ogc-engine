set shell := ["sh", "-c"]
BASE := "target/powerpc-unknown-eabi/debug/examples"

build:
    cargo build --examples

run EXAMPLE:
    cargo build --example {{EXAMPLE}}
    if [ -z {{BASE}}/{{EXAMPLE}}.elf ]; then rm {{BASE}}/{{EXAMPLE}}.elf; fi;
    cp {{BASE}}/{{EXAMPLE}} {{BASE}}/{{EXAMPLE}}.elf
    dolphin-emu -a LLE -e {{BASE}}/{{EXAMPLE}}.elf
