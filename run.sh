#!/bin/sh

set -ex

# Build bootloader
pushd bootloader && cargo build
popd

# Build kernel
pushd kernel && cargo build
popd

# Make EFI system partition
mkdir -p esp/efi/boot
cp target/x86_64-unknown-uefi/debug/bootloader.efi esp/efi/boot/bootx64.efi
cp target/x86_64-custom/debug/kernel esp/kernel.elf

# Launch VM
qemu-system-x86_64 \
  -serial stdio \
  -drive if=pflash,format=raw,readonly=on,file=assets/OVMF_CODE.fd \
  -drive if=pflash,format=raw,readonly=on,file=assets/OVMF_VARS.fd \
  -drive format=raw,file=fat:rw:esp \
  -device qemu-xhci
