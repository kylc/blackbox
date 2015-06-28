use hal::register::Register;

pub struct Rcc {
    pub cr: Cr,
    pub cfgr: Register<u32>,
    pub cir: Register<u32>,
    pub apb2rstr: Register<u32>,
    pub apb1rstr: Register<u32>,
    pub ahbenr: AhbEnr,
    pub apb2enr: Apb2Enr,
    pub apb1enr: Apb1Enr,
    pub bdcr: Register<u32>,
    pub csr: Register<u32>,
    pub ahbrstr: Register<u32>,
    pub cfgr2: Register<u32>,
    pub cfgr3: Register<u32>
}

register!(Cr;
    hsi_on,   set_hsi_on,   0..0;
    hsi_rdy,  set_hsi_rdy,  1..1;
    hsi_trim, set_hsi_trim, 3..7;
    hsi_cal,  set_hsi_cal,  8..15;
    hse_on,   set_hse_on,   16..16;
    hse_rdy,  set_hse_rdy,  17..17;
    hse_byp,  set_hse_byp,  18..18;
    css_on,   set_css_on,   19..19;
    pll_on,   set_pll_on,   24..24;
    pll_rdy,  set_pll_rdy,  25..25
);

register!(AhbEnr;
    dma1_en,  set_dma1_en,  0..0;
    dma2_en,  set_dma2_en,  1..1;
    sram_en,  set_sram_en,  2..2;
    flitf_en, set_flitf_en, 4..4;
    fmc_en,   set_fmc_en,   5..5;
    crc_en,   set_crc_en,   6..6;
    gpioa_en, set_gpioa_en, 17..17;
    gpiob_en, set_gpiob_en, 18..18;
    gpioc_en, set_gpioc_en, 19..19;
    gpiod_en, set_gpiod_en, 20..20;
    gpioe_en, set_gpioe_en, 21..21;
    gpiof_en, set_gpiof_en, 22..22;
    gpiog_en, set_gpiog_en, 23..23; // Only on STM32F303 xD xE.
    tsc_en,   set_tsc_en,   24..24;
    adc12_en, set_adc12_en, 28..28;
    adc34_en, set_adc34_en, 29..29
);

register!(Apb2Enr;
    sys_cfg_en, set_sys_cfg_en, 0..0;
    tim1_en,    set_tim1_en,    11..11;
    spi1_en,    set_spi1_en,    12..12;
    tim8_en,    set_tim8_en,    13..13;
    usart1_en,  set_usart1_en,  14..14;
    spi4_en,    set_spi4_en,    15..15;
    tim15_en,   set_tim15_en,   16..16;
    tim16_en,   set_tim16_en,   17..17;
    tim17_en,   set_tim17_en,   18..18;
    tim20_en,   set_tim20_en,   20..20
);

register!(Apb1Enr;
    tim2_en,   set_tim2_en,   0..0;
    tim3_en,   set_tim3_en,   1..1;
    tim4_en,   set_tim4_en,   2..2;
    tim6_en,   set_tim6_en,   4..4;
    tim7_en,   set_tim7_en,   5..5;
    wwdg_en,   set_wwdg_en,   11..11;
    spi2_en,   set_spi2_en,   14..14;
    spi3_en,   set_spi3_en,   15..15;
    usart2_en, set_usart2_en, 17..17;
    usart3_en, set_usart3_en, 18..18;
    uart4_en,  set_uart4_en,  19..19;
    uart5_en,  set_uart5_en,  20..20;
    i2c1_en,   set_i2c1_en,   21..21;
    i2c2_en,   set_i2c2_en,   22..22;
    usb_en,    set_usb_en,    23..23;
    can_en,    set_can_en,    25..25;
    dac2_en,   set_dac2_en,   26..26;
    pwr_en,    set_pwr_en,    28..28;
    dac1_en,   set_dac1_en,   29..29;
    i2c3_en,   set_i2c3_en,   30..30
);

