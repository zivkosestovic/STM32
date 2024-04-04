/* memory.x - Linker script for the STM32H753ZI */

MEMORY
{
  FLASH   (rx)   : ORIGIN = 0x08000000, LENGTH = 2048K
  DTCMRAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 128K
  RAM     (xrw)  : ORIGIN = 0x24000000, LENGTH = 512K
  RAM_D2  (xrw)  : ORIGIN = 0x30000000, LENGTH = 288K
  RAM_D3  (xrw)  : ORIGIN = 0x38000000, LENGTH = 64K
  ITCMRAM (xrw)  : ORIGIN = 0x00000000, LENGTH = 64K
}

SECTIONS
{
    /* other sections */
    .ramfunc :
    {
        *(.ramfunc)
    } > RAM
}
