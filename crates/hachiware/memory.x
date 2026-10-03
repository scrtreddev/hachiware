/* RP2040 memory layout, INCLUDEd by cortex-m-rt's link.x.

   2 MiB of QSPI flash on a stock Pico 1, and the first contiguous 256 KiB
   bank of SRAM (the two 4 KiB scratch banks are left out).

   BOOT2 reserves the first 256 bytes of flash for the second-stage loader
   that the RP2040 boot ROM copies into SRAM and runs; it configures the QSPI
   interface for execute-in-place. embassy-rp supplies that loader and its
   link-rp.x places it here, so this file is pure memory layout. */
MEMORY {
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}
