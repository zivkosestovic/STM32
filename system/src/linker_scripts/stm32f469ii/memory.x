/* memory.x - Linker script for the STM32F469II */

MEMORY
{
  CCMRAM   (xrw)   : ORIGIN = 0x10000000,  LENGTH = 64K
  RAM      (xrw)   : ORIGIN = 0x20000000,  LENGTH = 320K
  FLASH    (rx)    : ORIGIN = 0x8000000,   LENGTH = 2048K
}

SECTIONS
{
    /* other sections */
    .ramfunc :
    {
        *(.ramfunc)
    } > RAM
}
