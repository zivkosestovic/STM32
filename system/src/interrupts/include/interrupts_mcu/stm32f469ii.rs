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
pub const INTERRUPTS_NMI            :u32 = 2;
pub const INTERRUPTS_HARDFAULT      :u32 = 3;
pub const INTERRUPTS_MEMMANAGE      :u32 = 4;
pub const INTERRUPTS_BUSFAULT       :u32 = 5;
pub const INTERRUPTS_USAGEFAULT     :u32 = 6;
pub const INTERRUPTS_RESERVED1      :u32 = 7;
pub const INTERRUPTS_RESERVED2      :u32 = 8;
pub const INTERRUPTS_RESERVED3      :u32 = 9;
pub const INTERRUPTS_RESERVED4      :u32 = 10;
pub const INTERRUPTS_SVCALL         :u32 = 11;
pub const INTERRUPTS_DEBUGMON       :u32 = 12;
pub const INTERRUPTS_RESERVED5      :u32 = 13;
pub const INTERRUPTS_PENDSV         :u32 = 14;
pub const INTERRUPTS_SYSTICK        :u32 = 15;
pub const INTERRUPTS_WWDG           :u32 = 16;
pub const INTERRUPTS_PVD            :u32 = 17;
pub const INTERRUPTS_TAMP_STAMP     :u32 = 18;
pub const INTERRUPTS_RTC_WKUP       :u32 = 19;
pub const INTERRUPTS_FLASH          :u32 = 20;
pub const INTERRUPTS_RCC            :u32 = 21;
pub const INTERRUPTS_EXTI0          :u32 = 22;
pub const INTERRUPTS_EXTI1          :u32 = 23;
pub const INTERRUPTS_EXTI2          :u32 = 24;
pub const INTERRUPTS_EXTI3          :u32 = 25;
pub const INTERRUPTS_EXTI4          :u32 = 26;
pub const INTERRUPTS_DMA1_STREAM0   :u32 = 27;
pub const INTERRUPTS_DMA1_STREAM1   :u32 = 28;
pub const INTERRUPTS_DMA1_STREAM2   :u32 = 29;
pub const INTERRUPTS_DMA1_STREAM3   :u32 = 30;
pub const INTERRUPTS_DMA1_STREAM4   :u32 = 31;
pub const INTERRUPTS_DMA1_STREAM5   :u32 = 32;
pub const INTERRUPTS_DMA1_STREAM6   :u32 = 33;
pub const INTERRUPTS_ADC            :u32 = 34;
pub const INTERRUPTS_CAN1_TX        :u32 = 35;
pub const INTERRUPTS_CAN1_RX0       :u32 = 36;
pub const INTERRUPTS_CAN1_RX1       :u32 = 37;
pub const INTERRUPTS_CAN1_SCE       :u32 = 38;
pub const INTERRUPTS_EXTI9_5        :u32 = 39;
pub const INTERRUPTS_TIM1_BRK_TIM9  :u32 = 40;
pub const INTERRUPTS_TIM1_UP_TIM10  :u32 = 41;
pub const INTERRUPTS_TIM1_TRG_COM_TIM11 :u32 = 42;
pub const INTERRUPTS_TIM1_CC        :u32 = 43;
pub const INTERRUPTS_TIM2           :u32 = 44;
pub const INTERRUPTS_TIM3           :u32 = 45;
pub const INTERRUPTS_TIM4           :u32 = 46;
pub const INTERRUPTS_I2C1_EV        :u32 = 47;
pub const INTERRUPTS_I2C1_ER        :u32 = 48;
pub const INTERRUPTS_I2C2_EV        :u32 = 49;
pub const INTERRUPTS_I2C2_ER        :u32 = 50;
pub const INTERRUPTS_SPI1           :u32 = 51;
pub const INTERRUPTS_SPI2           :u32 = 52;
pub const INTERRUPTS_USART1         :u32 = 53;
pub const INTERRUPTS_USART2         :u32 = 54;
pub const INTERRUPTS_USART3         :u32 = 55;
pub const INTERRUPTS_EXTI15_10      :u32 = 56;
pub const INTERRUPTS_RTC_ALARM      :u32 = 57;
pub const INTERRUPTS_OTG_FS_WKUP    :u32 = 58;
pub const INTERRUPTS_TIM8_BRK_TIM12 :u32 = 59;
pub const INTERRUPTS_TIM8_UP_TIM13  :u32 = 60;
pub const INTERRUPTS_TIM8_TRG_COM_TIM14 :u32 = 61;
pub const INTERRUPTS_TIM8_CC        :u32 = 62;
pub const INTERRUPTS_DMA1_STREAM7   :u32 = 63;
pub const INTERRUPTS_FMC            :u32 = 64;
pub const INTERRUPTS_SDIO           :u32 = 65;
pub const INTERRUPTS_TIM5           :u32 = 66;
pub const INTERRUPTS_SPI3           :u32 = 67;
pub const INTERRUPTS_UART4          :u32 = 68;
pub const INTERRUPTS_UART5          :u32 = 69;
pub const INTERRUPTS_TIM6_DAC       :u32 = 70;
pub const INTERRUPTS_TIM7           :u32 = 71;
pub const INTERRUPTS_DMA2_STREAM0   :u32 = 72;
pub const INTERRUPTS_DMA2_STREAM1   :u32 = 73;
pub const INTERRUPTS_DMA2_STREAM2   :u32 = 74;
pub const INTERRUPTS_DMA2_STREAM3   :u32 = 75;
pub const INTERRUPTS_DMA2_STREAM4   :u32 = 76;
pub const INTERRUPTS_ETH            :u32 = 77;
pub const INTERRUPTS_ETH_WKUP       :u32 = 78;
pub const INTERRUPTS_CAN2_TX        :u32 = 79;
pub const INTERRUPTS_CAN2_RX0       :u32 = 80;
pub const INTERRUPTS_CAN2_RX1       :u32 = 81;
pub const INTERRUPTS_CAN2_SCE       :u32 = 82;
pub const INTERRUPTS_OTG_FS         :u32 = 83;
pub const INTERRUPTS_DMA2_STREAM5   :u32 = 84;
pub const INTERRUPTS_DMA2_STREAM6   :u32 = 85;
pub const INTERRUPTS_DMA2_STREAM7   :u32 = 86;
pub const INTERRUPTS_USART6         :u32 = 87;
pub const INTERRUPTS_I2C3_EV        :u32 = 88;
pub const INTERRUPTS_I2C3_ER        :u32 = 89;
pub const INTERRUPTS_OTG_HS_EP1_OUT :u32 = 90;
pub const INTERRUPTS_OTG_HS_EP1_IN  :u32 = 91;
pub const INTERRUPTS_OTG_HS_WKUP    :u32 = 92;
pub const INTERRUPTS_OTG_HS         :u32 = 93;
pub const INTERRUPTS_DCMI           :u32 = 94;
pub const INTERRUPTS_CRYP           :u32 = 95;
pub const INTERRUPTS_HASH_RNG       :u32 = 96;
pub const INTERRUPTS_FPU            :u32 = 97;
pub const INTERRUPTS_UART7          :u32 = 98;
pub const INTERRUPTS_UART8          :u32 = 99;
pub const INTERRUPTS_SPI4           :u32 = 100;
pub const INTERRUPTS_SPI5           :u32 = 101;
pub const INTERRUPTS_SPI6           :u32 = 102;
pub const INTERRUPTS_SAI1           :u32 = 103;
pub const INTERRUPTS_LCD_TFT        :u32 = 104;
pub const INTERRUPTS_LCD_TFT_ERROR  :u32 = 105;
pub const INTERRUPTS_DMA2D          :u32 = 106;
pub const INTERRUPTS_QUADSPI        :u32 = 107;
pub const INTERRUPTS_DSI            :u32 = 108;
// EOF Interrupt table

// Interrupt addresses
// No interrupt registers for stm32f411re.
// EOF Interrupt addresses

// Interrupt register bit values
// No interrupt bits for stm32f411re.
// EOF Interrupt register bit values

// ------------------------------------------------------------------------- END
