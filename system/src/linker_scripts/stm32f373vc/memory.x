/* memory.x - Linker script for the STM32F373VC */

MEMORY
{
  RAM      (xrw)   : ORIGIN = 0x20000000,  LENGTH = 32K
  FLASH    (rx)    : ORIGIN = 0x8000000,   LENGTH = 256K
}

SECTIONS
{
    /* other sections */
    .ramfunc :
    {
        *(.ramfunc)
    } > RAM
}
