/* memory.x - Linker script for the STM32F217ZG */

MEMORY
{
  RAM      (xrw)   : ORIGIN = 0x20000000,  LENGTH = 128K
  FLASH    (rx)    : ORIGIN = 0x8000000,   LENGTH = 1024K
}

SECTIONS
{
    /* other sections */
    .ramfunc :
    {
        *(.ramfunc)
    } > RAM
}
