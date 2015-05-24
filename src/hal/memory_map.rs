pub const FLASH_BASE: usize         = 0x08000000; /* FLASH(up to 256KB) base address in the alias region  */
pub const CCMDATARAM_BASE: usize    = 0x10000000; /* CCM(core coupled memory) data RAM(8 KB) base address in the alias region */
pub const SRAM_BASE: usize          = 0x20000000; /* SRAM(up to 40KB) base address in the alias region  */
pub const PERIPH_BASE: usize        = 0x40000000; /* Peripheral base address in the alias region   */

pub const CCMDATARAM_BB_BASE: usize = 0x12000000; /* CCM(core coupled memory) data RAM(8 KB) base address in the bit-band region */
pub const SRAM_BB_BASE: usize       = 0x22000000; /* SRAM(up to 40KB) base address in the bit-band region  */
pub const PERIPH_BB_BASE: usize     = 0x42000000; /* Peripheral base address in the bit-band region  */

/* Peripheral memory map */
pub const APB1PERIPH_BASE: usize    = PERIPH_BASE;
pub const APB2PERIPH_BASE: usize    = PERIPH_BASE + 0x00010000;
pub const AHB1PERIPH_BASE: usize    = PERIPH_BASE + 0x00020000;
pub const AHB2PERIPH_BASE: usize    = PERIPH_BASE + 0x08000000;
pub const AHB3PERIPH_BASE: usize    = PERIPH_BASE + 0x10000000;

/* APB1 peripherals */
pub const TIM2_BASE: usize          = APB1PERIPH_BASE + 0x00000000;
pub const TIM3_BASE: usize          = APB1PERIPH_BASE + 0x00000400;
pub const TIM4_BASE: usize          = APB1PERIPH_BASE + 0x00000800;
pub const TIM6_BASE: usize          = APB1PERIPH_BASE + 0x00001000;
pub const TIM7_BASE: usize          = APB1PERIPH_BASE + 0x00001400;
pub const RTC_BASE: usize           = APB1PERIPH_BASE + 0x00002800;
pub const WWDG_BASE: usize          = APB1PERIPH_BASE + 0x00002C00;
pub const IWDG_BASE: usize          = APB1PERIPH_BASE + 0x00003000;
pub const I2S2EXT_BASE: usize       = APB1PERIPH_BASE + 0x00003400;
pub const SPI2_BASE: usize          = APB1PERIPH_BASE + 0x00003800;
pub const SPI3_BASE: usize          = APB1PERIPH_BASE + 0x00003C00;
pub const I2S3EXT_BASE: usize       = APB1PERIPH_BASE + 0x00004000;
pub const USART2_BASE: usize        = APB1PERIPH_BASE + 0x00004400;
pub const USART3_BASE: usize        = APB1PERIPH_BASE + 0x00004800;
pub const UART4_BASE: usize         = APB1PERIPH_BASE + 0x00004C00;
pub const UART5_BASE: usize         = APB1PERIPH_BASE + 0x00005000;
pub const I2C1_BASE: usize          = APB1PERIPH_BASE + 0x00005400;
pub const I2C2_BASE: usize          = APB1PERIPH_BASE + 0x00005800;
pub const USB_BASE: usize           = APB1PERIPH_BASE + 0x00005C00; /* USB_IP Peripheral Registers base address */
pub const USB_PMAADDR: usize        = APB1PERIPH_BASE + 0x00006000; /* USB_IP Packet Memory Area base address */
pub const CAN_BASE: usize           = APB1PERIPH_BASE + 0x00006400;
pub const PWR_BASE: usize           = APB1PERIPH_BASE + 0x00007000;
pub const DAC1_BASE: usize          = APB1PERIPH_BASE + 0x00007400;
pub const DAC_BASE: usize           = DAC1_BASE;

/* APB2 peripherals */
pub const SYSCFG_BASE: usize        = APB2PERIPH_BASE + 0x00000000;
pub const COMP1_BASE: usize         = APB2PERIPH_BASE + 0x0000001C;
pub const COMP2_BASE: usize         = APB2PERIPH_BASE + 0x00000020;
pub const COMP3_BASE: usize         = APB2PERIPH_BASE + 0x00000024;
pub const COMP4_BASE: usize         = APB2PERIPH_BASE + 0x00000028;
pub const COMP5_BASE: usize         = APB2PERIPH_BASE + 0x0000002C;
pub const COMP6_BASE: usize         = APB2PERIPH_BASE + 0x00000030;
pub const COMP7_BASE: usize         = APB2PERIPH_BASE + 0x00000034;
pub const COMP_BASE: usize          = COMP1_BASE;
pub const OPAMP1_BASE: usize        = APB2PERIPH_BASE + 0x00000038;
pub const OPAMP2_BASE: usize        = APB2PERIPH_BASE + 0x0000003C;
pub const OPAMP3_BASE: usize        = APB2PERIPH_BASE + 0x00000040;
pub const OPAMP4_BASE: usize        = APB2PERIPH_BASE + 0x00000044;
pub const OPAMP_BASE:usize          = OPAMP1_BASE;
pub const EXTI_BASE: usize          = APB2PERIPH_BASE + 0x00000400;
pub const TIM1_BASE: usize          = APB2PERIPH_BASE + 0x00002C00;
pub const SPI1_BASE: usize          = APB2PERIPH_BASE + 0x00003000;
pub const TIM8_BASE: usize          = APB2PERIPH_BASE + 0x00003400;
pub const USART1_BASE: usize        = APB2PERIPH_BASE + 0x00003800;
pub const TIM15_BASE: usize         = APB2PERIPH_BASE + 0x00004000;
pub const TIM16_BASE: usize         = APB2PERIPH_BASE + 0x00004400;
pub const TIM17_BASE: usize         = APB2PERIPH_BASE + 0x00004800;

/* AHB1 peripherals */
pub const DMA1_BASE: usize          = AHB1PERIPH_BASE + 0x00000000;
pub const DMA1_CHANNEL1_BASE: usize = AHB1PERIPH_BASE + 0x00000008;
pub const DMA1_CHANNEL2_BASE: usize = AHB1PERIPH_BASE + 0x0000001C;
pub const DMA1_CHANNEL3_BASE: usize = AHB1PERIPH_BASE + 0x00000030;
pub const DMA1_CHANNEL4_BASE: usize = AHB1PERIPH_BASE + 0x00000044;
pub const DMA1_CHANNEL5_BASE: usize = AHB1PERIPH_BASE + 0x00000058;
pub const DMA1_CHANNEL6_BASE: usize = AHB1PERIPH_BASE + 0x0000006C;
pub const DMA1_CHANNEL7_BASE: usize = AHB1PERIPH_BASE + 0x00000080;
pub const DMA2_BASE: usize          = AHB1PERIPH_BASE + 0x00000400;
pub const DMA2_CHANNEL1_BASE: usize = AHB1PERIPH_BASE + 0x00000408;
pub const DMA2_CHANNEL2_BASE: usize = AHB1PERIPH_BASE + 0x0000041C;
pub const DMA2_CHANNEL3_BASE: usize = AHB1PERIPH_BASE + 0x00000430;
pub const DMA2_CHANNEL4_BASE: usize = AHB1PERIPH_BASE + 0x00000444;
pub const DMA2_CHANNEL5_BASE: usize = AHB1PERIPH_BASE + 0x00000458;
pub const RCC_BASE: usize           = AHB1PERIPH_BASE + 0x00001000;
pub const FLASH_R_BASE: usize       = AHB1PERIPH_BASE + 0x00002000; /* Flash registers base address */
pub const OB_BASE: usize            = 0x1FFFF800;                   /* Flash Option Bytes base address */
pub const CRC_BASE: usize           = AHB1PERIPH_BASE + 0x00003000;
pub const TSC_BASE: usize           = AHB1PERIPH_BASE + 0x00004000;

/* AHB2 peripherals */
pub const GPIOA_BASE: usize         = AHB2PERIPH_BASE + 0x00000000;
pub const GPIOB_BASE: usize         = AHB2PERIPH_BASE + 0x00000400;
pub const GPIOC_BASE: usize         = AHB2PERIPH_BASE + 0x00000800;
pub const GPIOD_BASE: usize         = AHB2PERIPH_BASE + 0x00000C00;
pub const GPIOE_BASE: usize         = AHB2PERIPH_BASE + 0x00001000;
pub const GPIOF_BASE: usize         = AHB2PERIPH_BASE + 0x00001400;

/* AHB3 peripherals */
pub const ADC1_BASE: usize          = AHB3PERIPH_BASE + 0x00000000;
pub const ADC2_BASE: usize          = AHB3PERIPH_BASE + 0x00000100;
pub const ADC1_2_COMMON_BASE: usize = AHB3PERIPH_BASE + 0x00000300;
pub const ADC3_BASE: usize          = AHB3PERIPH_BASE + 0x00000400;
pub const ADC4_BASE: usize          = AHB3PERIPH_BASE + 0x00000500;
pub const ADC3_4_COMMON_BASE: usize = AHB3PERIPH_BASE + 0x00000700;

pub const DBGMCU_BASE: usize        = 0xE0042000; /* Debug MCU registers base address */
