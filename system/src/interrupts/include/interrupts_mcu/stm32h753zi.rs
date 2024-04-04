/****************************************************************************
**
** Copyright (C) 2024 MikroElektronika d.o.o.
** Contact: https://www.mikroe.com/contact
**
** This file is part of the mikroSDK package
**
** Commercial License Usage
**
** Licensees holding valid commercial NECTO compilers AI licenses may use this
** file in accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The MikroElektronika Company.
** For licensing terms and conditions see
** https://www.mikroe.com/legal/software-license-agreement.
** For further information use the contact form at
** https://www.mikroe.com/contact.
**
**
** GNU Lesser General Public License Usage
**
** Alternatively, this file may be used for
** non-commercial projects under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** The above copyright notice and this permission notice shall be
** included in all copies or substantial portions of the Software.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
** OF MERCHANTABILITY, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
** TO THE WARRANTIES FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
** IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
** DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT
** OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
** OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
**
****************************************************************************/

// Interrupt table
pub const INTERRUPTS_NMI                    :u32 = 2;
pub const INTERRUPTS_HARDFAULT              :u32 = 3;
pub const INTERRUPTS_MEMMANAGE              :u32 = 4;
pub const INTERRUPTS_BUSFAULT               :u32 = 5;
pub const INTERRUPTS_USAGEFAULT             :u32 = 6;
pub const INTERRUPTS_RESERVED1              :u32 = 7;
pub const INTERRUPTS_RESERVED2              :u32 = 8;
pub const INTERRUPTS_RESERVED3              :u32 = 9;
pub const INTERRUPTS_RESERVED4              :u32 = 10;
pub const INTERRUPTS_SVCALL                 :u32 = 11;
pub const INTERRUPTS_DEBUGMON               :u32 = 12;
pub const INTERRUPTS_RESERVED5              :u32 = 13;
pub const INTERRUPTS_PENDSV                 :u32 = 14;
pub const INTERRUPTS_SYSTICK                :u32 = 15;
pub const INTERRUPTS_WWDG1                  :u32 = 16;
pub const INTERRUPTS_PVD_PVM                :u32 = 17;
pub const INTERRUPTS_RTC_TAMP_STAMP_CSS_LSE :u32 = 18;
pub const INTERRUPTS_RTC_WKUP               :u32 = 19;
pub const INTERRUPTS_FLASH                  :u32 = 20;
pub const INTERRUPTS_RCC                    :u32 = 21;
pub const INTERRUPTS_EXTI0                  :u32 = 22;
pub const INTERRUPTS_EXTI1                  :u32 = 23;
pub const INTERRUPTS_EXTI2                  :u32 = 24;
pub const INTERRUPTS_EXTI3                  :u32 = 25;
pub const INTERRUPTS_EXTI4                  :u32 = 26;
pub const INTERRUPTS_DMA_STR0               :u32 = 27;
pub const INTERRUPTS_DMA_STR1               :u32 = 28;
pub const INTERRUPTS_DMA_STR2               :u32 = 29;
pub const INTERRUPTS_DMA_STR3               :u32 = 30;
pub const INTERRUPTS_DMA_STR4               :u32 = 31;
pub const INTERRUPTS_DMA_STR5               :u32 = 32;
pub const INTERRUPTS_DMA_STR6               :u32 = 33;
pub const INTERRUPTS_ADC1_2                 :u32 = 34;
pub const INTERRUPTS_FDCAN1_IT0             :u32 = 35;
pub const INTERRUPTS_FDCAN2_IT0             :u32 = 36;
pub const INTERRUPTS_FDCAN1_IT1             :u32 = 37;
pub const INTERRUPTS_FDCAN2_IT1             :u32 = 38;
pub const INTERRUPTS_EXTI9_5                :u32 = 39;
pub const INTERRUPTS_TIM1_BRK               :u32 = 40;
pub const INTERRUPTS_TIM1_UP                :u32 = 41;
pub const INTERRUPTS_TIM1_TRG_COM           :u32 = 42;
pub const INTERRUPTS_TIM_CC                 :u32 = 43;
pub const INTERRUPTS_TIM2                   :u32 = 44;
pub const INTERRUPTS_TIM3                   :u32 = 45;
pub const INTERRUPTS_TIM4                   :u32 = 46;
pub const INTERRUPTS_I2C1_EV                :u32 = 47;
pub const INTERRUPTS_I2C1_ER                :u32 = 48;
pub const INTERRUPTS_I2C2_EV                :u32 = 49;
pub const INTERRUPTS_I2C2_ER                :u32 = 50;
pub const INTERRUPTS_SPI1                   :u32 = 51;
pub const INTERRUPTS_SPI2                   :u32 = 52;
pub const INTERRUPTS_USART1                 :u32 = 53;
pub const INTERRUPTS_USART2                 :u32 = 54;
pub const INTERRUPTS_USART3                 :u32 = 55;
pub const INTERRUPTS_EXTI15_10              :u32 = 56;
pub const INTERRUPTS_RTC_ALARM              :u32 = 57;
pub const INTERRUPTS_RESERVED6              :u32 = 58;
pub const INTERRUPTS_TIM8_BRK_TIM12         :u32 = 59;
pub const INTERRUPTS_TIM8_UP_TIM13          :u32 = 60;
pub const INTERRUPTS_TIM8_TRG_COM_TIM14     :u32 = 61;
pub const INTERRUPTS_TIM8_CC                :u32 = 62;
pub const INTERRUPTS_DMA1_STR7              :u32 = 63;
pub const INTERRUPTS_FMC                    :u32 = 64;
pub const INTERRUPTS_SDMMC1                 :u32 = 65;
pub const INTERRUPTS_TIM5                   :u32 = 66;
pub const INTERRUPTS_SPI3                   :u32 = 67;
pub const INTERRUPTS_UART4                  :u32 = 68;
pub const INTERRUPTS_UART5                  :u32 = 69;
pub const INTERRUPTS_TIM6_DAC               :u32 = 70;
pub const INTERRUPTS_TIM7                   :u32 = 71;
pub const INTERRUPTS_DMA2_STR0              :u32 = 72;
pub const INTERRUPTS_DMA2_STR1              :u32 = 73;
pub const INTERRUPTS_DMA2_STR2              :u32 = 74;
pub const INTERRUPTS_DMA2_STR3              :u32 = 75;
pub const INTERRUPTS_DMA2_STR4              :u32 = 76;
pub const INTERRUPTS_ETH                    :u32 = 77;
pub const INTERRUPTS_ETH_WKUP               :u32 = 78;
pub const INTERRUPTS_FDCAN_CAL              :u32 = 79;
pub const INTERRUPTS_RESERVED7              :u32 = 80;
pub const INTERRUPTS_RESERVED8              :u32 = 81;
pub const INTERRUPTS_RESERVED9              :u32 = 82;
pub const INTERRUPTS_RESERVED10             :u32 = 83;
pub const INTERRUPTS_DMA2_STR5              :u32 = 84;
pub const INTERRUPTS_DMA2_STR6              :u32 = 85;
pub const INTERRUPTS_DMA2_STR7              :u32 = 86;
pub const INTERRUPTS_USART6                 :u32 = 87;
pub const INTERRUPTS_I2C3_EV                :u32 = 88;
pub const INTERRUPTS_I2C3_ER                :u32 = 89;
pub const INTERRUPTS_OTG_HP_EP1_OUT         :u32 = 90;
pub const INTERRUPTS_OTG_HS_EP1_IN          :u32 = 91;
pub const INTERRUPTS_OTG_HS_WKUP            :u32 = 92;
pub const INTERRUPTS_OTH_HS                 :u32 = 93;
pub const INTERRUPTS_DCMI                   :u32 = 94;
pub const INTERRUPTS_CRYP                   :u32 = 95;
pub const INTERRUPTS_HASH_RNG               :u32 = 96;
pub const INTERRUPTS_FPU                    :u32 = 97;
pub const INTERRUPTS_UART7                  :u32 = 98;
pub const INTERRUPTS_UART8                  :u32 = 99;
pub const INTERRUPTS_SPI4                   :u32 = 100;
pub const INTERRUPTS_SPI5                   :u32 = 101;
pub const INTERRUPTS_SPI6                   :u32 = 102;
pub const INTERRUPTS_SAI1                   :u32 = 103;
pub const INTERRUPTS_LTDC                   :u32 = 104;
pub const INTERRUPTS_LTDC_ER                :u32 = 105;
pub const INTERRUPTS_DMA2D                  :u32 = 106;
pub const INTERRUPTS_SAI2                   :u32 = 107;
pub const INTERRUPTS_QUADSPI                :u32 = 108;
pub const INTERRUPTS_LPTIM1                 :u32 = 109;
pub const INTERRUPTS_CEC                    :u32 = 110;
pub const INTERRUPTS_I2C4_EV                :u32 = 111;
pub const INTERRUPTS_I2C4_ER                :u32 = 112;
pub const INTERRUPTS_SPDIF                  :u32 = 113;
pub const INTERRUPTS_OTG_FS_EP1_OUT         :u32 = 114;
pub const INTERRUPTS_OTG_FS_EP1_IN          :u32 = 115;
pub const INTERRUPTS_OTG_FS_WKUP            :u32 = 116;
pub const INTERRUPTS_OTG_FS                 :u32 = 117;
pub const INTERRUPTS_DMAMUX1_OV             :u32 = 118;
pub const INTERRUPTS_HRTIM1_MST             :u32 = 119;
pub const INTERRUPTS_HRTIM1_TIMA            :u32 = 120;
pub const INTERRUPTS_HRTIM_TIMB             :u32 = 121;
pub const INTERRUPTS_HRTIM1_TIMC            :u32 = 122;
pub const INTERRUPTS_HRTIM1_TIMD            :u32 = 123;
pub const INTERRUPTS_HRTIM_TIME             :u32 = 124;
pub const INTERRUPTS_HRTIM1_FLT             :u32 = 125;
pub const INTERRUPTS_DFSDM1_FLT0            :u32 = 126;
pub const INTERRUPTS_DFSDM1_FLT1            :u32 = 127;
pub const INTERRUPTS_DFSDM1_FLT2            :u32 = 128;
pub const INTERRUPTS_DFSDM1_FLT3            :u32 = 129;
pub const INTERRUPTS_SAI3                   :u32 = 130;
pub const INTERRUPTS_SWPMI1                 :u32 = 131;
pub const INTERRUPTS_TIM15                  :u32 = 132;
pub const INTERRUPTS_TIM16                  :u32 = 133;
pub const INTERRUPTS_MDIOS_WKUP             :u32 = 134;
pub const INTERRUPTS_MDIOS                  :u32 = 135;
pub const INTERRUPTS_JPEG                   :u32 = 136;
pub const INTERRUPTS_MDMA                   :u32 = 137;
pub const INTERRUPTS_SDMMC2                 :u32 = 138;
pub const INTERRUPTS_HSEM0                  :u32 = 139;
pub const INTERRUPTS_RESERVED11             :u32 = 140;
pub const INTERRUPTS_ADC3                   :u32 = 141;
pub const INTERRUPTS_DMAMUX2_OVR            :u32 = 142;
pub const INTERRUPTS_BDMA_CH0               :u32 = 143;
pub const INTERRUPTS_BDMA_CH1               :u32 = 144;
pub const INTERRUPTS_BDMA_CH2               :u32 = 145;
pub const INTERRUPTS_BDMA_CH3               :u32 = 146;
pub const INTERRUPTS_BDMA_CH4               :u32 = 147;
pub const INTERRUPTS_BDMA_CH5               :u32 = 148;
pub const INTERRUPTS_BDMA_CH6               :u32 = 149;
pub const INTERRUPTS_BDMA_CH7               :u32 = 150;
pub const INTERRUPTS_COMP                   :u32 = 151;
pub const INTERRUPTS_LPTIM2                 :u32 = 152;
pub const INTERRUPTS_LPTIM3                 :u32 = 153;
pub const INTERRUPTS_LPTIM4                 :u32 = 154;
pub const INTERRUPTS_LPTIM5                 :u32 = 155;
pub const INTERRUPTS_LPUART                 :u32 = 156;
pub const INTERRUPTS_WWDG1_RST              :u32 = 157;
pub const INTERRUPTS_CRS                    :u32 = 158;
pub const INTERRUPTS_RAMECC1                :u32 = 159;
pub const INTERRUPTS_RAMECC2                :u32 = 160;
pub const INTERRUPTS_RAMECC3                :u32 = 161;
pub const INTERRUPTS_SAI4                   :u32 = 162;
pub const INTERRUPTS_RESERVED12             :u32 = 163;
pub const INTERRUPTS_RESERVED13             :u32 = 164;
pub const INTERRUPTS_WKUP                   :u32 = 165;
// EOF Interrupt table

// Interrupt addresses
// No interrupt registers for stm32l152re.
// EOF Interrupt addresses

// Interrupt register bit values
// No interrupt bits for stm32l152re.
// EOF Interrupt register bit values

// ------------------------------------------------------------------------- END
