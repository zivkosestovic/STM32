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
pub const INTERRUPTS_NMI                 : u32 = 2;
pub const INTERRUPTS_HARDFAULT           : u32 = 3;
pub const INTERRUPTS_RESERVED1           : u32 = 4;
pub const INTERRUPTS_RESERVED2           : u32 = 5;
pub const INTERRUPTS_RESERVED3           : u32 = 6;
pub const INTERRUPTS_RESERVED4           : u32 = 7;
pub const INTERRUPTS_RESERVED5           : u32 = 8;
pub const INTERRUPTS_RESERVED6           : u32 = 9;
pub const INTERRUPTS_RESERVED7           : u32 = 10;
pub const INTERRUPTS_SVCALL              : u32 = 11;
pub const INTERRUPTS_RESERVED8           : u32 = 12;
pub const INTERRUPTS_RESERVED9           : u32 = 13;
pub const INTERRUPTS_PENDSV              : u32 = 14;
pub const INTERRUPTS_SYSTICK             : u32 = 15;
pub const INTERRUPTS_WWDG                : u32 = 16;
pub const INTERRUPTS_PVD_VDDIO2          : u32 = 17;
pub const INTERRUPTS_RTC                 : u32 = 18;
pub const INTERRUPTS_FLASH               : u32 = 19;
pub const INTERRUPTS_RCC_CRS             : u32 = 20;
pub const INTERRUPTS_EXTI0_1             : u32 = 21;
pub const INTERRUPTS_EXTI2_3             : u32 = 22;
pub const INTERRUPTS_EXTI4_15            : u32 = 23;
pub const INTERRUPTS_TSC                 : u32 = 24;
pub const INTERRUPTS_DMA_CH1             : u32 = 25;
pub const INTERRUPTS_DMA_CH2_3           : u32 = 26;
pub const INTERRUPTS_DMA_CH4_5_6_7       : u32 = 27;
pub const INTERRUPTS_ADC_COMP            : u32 = 28;
pub const INTERRUPTS_TIM1_BRK_UP_TRG_COM : u32 = 29;
pub const INTERRUPTS_TIM1_CC             : u32 = 30;
pub const INTERRUPTS_TIM2                : u32 = 31;
pub const INTERRUPTS_TIM3                : u32 = 32;
pub const INTERRUPTS_TIM6_DAC            : u32 = 33;
pub const INTERRUPTS_TIM7                : u32 = 34;
pub const INTERRUPTS_TIM14               : u32 = 35;
pub const INTERRUPTS_TIM15               : u32 = 36;
pub const INTERRUPTS_TIM16               : u32 = 37;
pub const INTERRUPTS_TIM17               : u32 = 38;
pub const INTERRUPTS_I2C1                : u32 = 39;
pub const INTERRUPTS_I2C2                : u32 = 40;
pub const INTERRUPTS_SPI1                : u32 = 41;
pub const INTERRUPTS_SPI2                : u32 = 42;
pub const INTERRUPTS_USART1              : u32 = 43;
pub const INTERRUPTS_USART2              : u32 = 44;
pub const INTERRUPTS_USART3_4            : u32 = 45;
pub const INTERRUPTS_CEC_CAN             : u32 = 46;
pub const INTERRUPTS_USB                 : u32 = 47;

// EOF Interrupt table

// Interrupt addresses
// No interrupt registers for stm32f411re.
// EOF Interrupt addresses

// Interrupt register bit values
// No interrupt bits for stm32f411re.
// EOF Interrupt register bit values

// ------------------------------------------------------------------------- END
