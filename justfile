set shell := ["sh", "-c"]
IP := "192.168.11.171"
BASE := "target/powerpc-unknown-eabi/debug/examples"

default:
    @just --list --unsorted

build EXAMPLE:
    cargo +nightly build --example {{EXAMPLE}}
    if [ -z {{BASE}}/{{EXAMPLE}}.elf ]; then @rm {{BASE}}/{{EXAMPLE}}.elf; fi;
    cp {{BASE}}/{{EXAMPLE}} {{BASE}}/{{EXAMPLE}}.elf

run EXAMPLE: (build EXAMPLE)
    dolphin-emu -a LLE -e {{BASE}}/{{EXAMPLE}}.elf

deploy EXAMPLE: (build EXAMPLE)
    $DEVKITPRO/tools/bin/elf2dol {{BASE}}/{{EXAMPLE}}.elf {{BASE}}/{{EXAMPLE}}.dol
    WIILOAD="tcp:{{IP}}" $DEVKITPRO/tools/bin/wiiload {{BASE}}/{{EXAMPLE}}.dol
