/* RP2040 memory layout, INCLUDEd by cortex-m-rt's link.x.
   2 MiB of QSPI flash on a stock Pico 1, 264 KiB of SRAM (the first 256 KiB
   striped as one contiguous bank; the two 4 KiB scratch banks are left out). */
MEMORY {
    /* The RP2040 boot ROM copies the first 256 bytes of flash into SRAM and
       runs them; that second-stage loader configures the QSPI interface for
       execute-in-place. Reserved here so .text starts after it. */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 256K
}

/* Supplied by rp2040-boot2 via the rp-pico BSP's default `boot2` feature. */
EXTERN(BOOT2_FIRMWARE)

SECTIONS {
    .boot2 ORIGIN(BOOT2) :
    {
        KEEP(*(.boot2));
    } > BOOT2
} INSERT BEFORE .text;
