/* memory.x - Linker script for the STM32L073RZ */

MEMORY
{
  RAM      (xrw)   : ORIGIN = 0x20000000,  LENGTH = 20K
  FLASH    (rx)    : ORIGIN = 0x8000000,   LENGTH = 192K
}

SECTIONS
{
    /* other sections */
    .ramfunc :
    {
        *(.ramfunc)
    } > RAM
}
