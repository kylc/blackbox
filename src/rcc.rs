use register::Register;

pub struct Rcc {
    pub cr: Register<u32>,
    pub cfgr: Register<u32>,
    pub cir: Register<u32>,
    pub apb2rstr: Register<u32>,
    pub apb1rstr: Register<u32>,
    pub ahbenr: RccAhbEnR,
    pub apb2enr: Register<u32>,
    pub apb1enr: Register<u32>,
    pub bdcr: Register<u32>,
    pub csr: Register<u32>,
    pub ahbrstr: Register<u32>,
    pub cfgr2: Register<u32>,
    pub cfgr3: Register<u32>
}

register!(RccAhbEnR;
    dma1_en,  set_dma1_en,  0;
    dma2_en,  set_dma2_en,  1;
    sram_en,  set_sram_en,  2;
    flitf_en, set_flitf_en, 4;
    fmc_en,   set_fmc_en,   5;
    crc_en,   set_crc_en,   6;
    gpioa_en, set_gpioa_en, 17;
    gpiob_en, set_gpiob_en, 18;
    gpioc_en, set_gpioc_en, 19;
    gpiod_en, set_gpiod_en, 20;
    gpioe_en, set_gpioe_en, 21;
    gpiof_en, set_gpiof_en, 22;
    gpiog_en, set_gpiog_en, 23; // Only on STM32F303 xD xE.
    tsc_en,   set_tsc_en,   24;
    adc12_en, set_adc12_en, 28;
    adc34_en, set_adc34_en, 29
);
