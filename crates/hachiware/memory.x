/* RP2350 memory layout, INCLUDEd by cortex-m-rt's link.x.

   4 MiB of QSPI flash on a stock Pico 2 W, and the 512 KiB of striped main
   SRAM (the two 4 KiB scratch banks, SRAM4 and SRAM5, are listed but unused).

   There is no BOOT2 region: the RP2350 boot ROM configures flash itself and
   instead looks for an IMAGE_DEF block within the first 4 KiB of the image.
   embassy-rp emits that block into `.start_block`, and the SECTIONS below put
   it right after the vector table. `.end_block` and the `start_to_end` /
   `end_to_start` symbols close the block loop the ROM walks. */
MEMORY {
    FLASH : ORIGIN = 0x10000000, LENGTH = 4096K
    RAM   : ORIGIN = 0x20000000, LENGTH = 512K
    SRAM4 : ORIGIN = 0x20080000, LENGTH = 4K
    SRAM5 : ORIGIN = 0x20081000, LENGTH = 4K
}

SECTIONS {
    .start_block : ALIGN(4)
    {
        __start_block_addr = .;
        KEEP(*(.start_block));
        KEEP(*(.boot_info));
    } > FLASH
} INSERT AFTER .vector_table;

/* Moves the start of .text past the block above. */
_stext = ADDR(.start_block) + SIZEOF(.start_block);

SECTIONS {
    .bi_entries : ALIGN(4)
    {
        __bi_entries_start = .;
        KEEP(*(.bi_entries));
        . = ALIGN(4);
        __bi_entries_end = .;
    } > FLASH
} INSERT AFTER .text;

SECTIONS {
    .end_block : ALIGN(4)
    {
        __end_block_addr = .;
        KEEP(*(.end_block));
    } > FLASH
} INSERT AFTER .uninit;

PROVIDE(start_to_end = __end_block_addr - __start_block_addr);
PROVIDE(end_to_start = __start_block_addr - __end_block_addr);
