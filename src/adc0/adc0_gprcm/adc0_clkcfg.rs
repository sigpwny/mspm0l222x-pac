#[doc = "Register `ADC0_CLKCFG` reader"]
pub type R = crate::R<Adc0ClkcfgSpec>;
#[doc = "Register `ADC0_CLKCFG` writer"]
pub type W = crate::W<Adc0ClkcfgSpec>;
#[doc = "ADC sample clock source selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sampclk {
    #[doc = "0: ULPCLK is the source of ADC sample clock."]
    Ulpclk = 0,
    #[doc = "1: SYSOSC is the source of ADC sample clock."]
    Sysosc = 1,
    #[doc = "2: HFCLK clock is the source of ADC sample clock. Note : HFCLK may not be available on all the devices."]
    Hfclk = 2,
}
impl From<Sampclk> for u8 {
    #[inline(always)]
    fn from(variant: Sampclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sampclk {
    type Ux = u8;
}
impl crate::IsEnum for Sampclk {}
#[doc = "Field `SAMPCLK` reader - ADC sample clock source selection."]
pub type SampclkR = crate::FieldReader<Sampclk>;
impl SampclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sampclk> {
        match self.bits {
            0 => Some(Sampclk::Ulpclk),
            1 => Some(Sampclk::Sysosc),
            2 => Some(Sampclk::Hfclk),
            _ => None,
        }
    }
    #[doc = "ULPCLK is the source of ADC sample clock."]
    #[inline(always)]
    pub fn is_ulpclk(&self) -> bool {
        *self == Sampclk::Ulpclk
    }
    #[doc = "SYSOSC is the source of ADC sample clock."]
    #[inline(always)]
    pub fn is_sysosc(&self) -> bool {
        *self == Sampclk::Sysosc
    }
    #[doc = "HFCLK clock is the source of ADC sample clock. Note : HFCLK may not be available on all the devices."]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == Sampclk::Hfclk
    }
}
#[doc = "Field `SAMPCLK` writer - ADC sample clock source selection."]
pub type SampclkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sampclk>;
impl<'a, REG> SampclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ULPCLK is the source of ADC sample clock."]
    #[inline(always)]
    pub fn ulpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sampclk::Ulpclk)
    }
    #[doc = "SYSOSC is the source of ADC sample clock."]
    #[inline(always)]
    pub fn sysosc(self) -> &'a mut crate::W<REG> {
        self.variant(Sampclk::Sysosc)
    }
    #[doc = "HFCLK clock is the source of ADC sample clock. Note : HFCLK may not be available on all the devices."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Sampclk::Hfclk)
    }
}
#[doc = "CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cconrun {
    #[doc = "0: ADC conversion clock source is not kept continuously on during RUN mode."]
    Disable = 0,
    #[doc = "1: ADC conversion clock source kept continuously on during RUN mode."]
    Enable = 1,
}
impl From<Cconrun> for bool {
    #[inline(always)]
    fn from(variant: Cconrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCONRUN` reader - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."]
pub type CconrunR = crate::BitReader<Cconrun>;
impl CconrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cconrun {
        match self.bits {
            false => Cconrun::Disable,
            true => Cconrun::Enable,
        }
    }
    #[doc = "ADC conversion clock source is not kept continuously on during RUN mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cconrun::Disable
    }
    #[doc = "ADC conversion clock source kept continuously on during RUN mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cconrun::Enable
    }
}
#[doc = "Field `CCONRUN` writer - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."]
pub type CconrunW<'a, REG> = crate::BitWriter<'a, REG, Cconrun>;
impl<'a, REG> CconrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC conversion clock source is not kept continuously on during RUN mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cconrun::Disable)
    }
    #[doc = "ADC conversion clock source kept continuously on during RUN mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cconrun::Enable)
    }
}
#[doc = "CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cconstop {
    #[doc = "0: ADC conversion clock source is not kept continuously on during STOP mode."]
    Disable = 0,
    #[doc = "1: ADC conversion clock source kept continuously on during STOP mode."]
    Enable = 1,
}
impl From<Cconstop> for bool {
    #[inline(always)]
    fn from(variant: Cconstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCONSTOP` reader - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."]
pub type CconstopR = crate::BitReader<Cconstop>;
impl CconstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cconstop {
        match self.bits {
            false => Cconstop::Disable,
            true => Cconstop::Enable,
        }
    }
    #[doc = "ADC conversion clock source is not kept continuously on during STOP mode."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cconstop::Disable
    }
    #[doc = "ADC conversion clock source kept continuously on during STOP mode."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cconstop::Enable
    }
}
#[doc = "Field `CCONSTOP` writer - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."]
pub type CconstopW<'a, REG> = crate::BitWriter<'a, REG, Cconstop>;
impl<'a, REG> CconstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC conversion clock source is not kept continuously on during STOP mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cconstop::Disable)
    }
    #[doc = "ADC conversion clock source kept continuously on during STOP mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cconstop::Enable)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC sample clock source selection."]
    #[inline(always)]
    pub fn sampclk(&self) -> SampclkR {
        SampclkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."]
    #[inline(always)]
    pub fn cconrun(&self) -> CconrunR {
        CconrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."]
    #[inline(always)]
    pub fn cconstop(&self) -> CconstopR {
        CconstopR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC sample clock source selection."]
    #[inline(always)]
    pub fn sampclk(&mut self) -> SampclkW<'_, Adc0ClkcfgSpec> {
        SampclkW::new(self, 0)
    }
    #[doc = "Bit 4 - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."]
    #[inline(always)]
    pub fn cconrun(&mut self) -> CconrunW<'_, Adc0ClkcfgSpec> {
        CconrunW::new(self, 4)
    }
    #[doc = "Bit 5 - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."]
    #[inline(always)]
    pub fn cconstop(&mut self) -> CconstopW<'_, Adc0ClkcfgSpec> {
        CconstopW::new(self, 5)
    }
}
#[doc = "ADC clock configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_clkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_clkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0ClkcfgSpec;
impl crate::RegisterSpec for Adc0ClkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_clkcfg::R`](R) reader structure"]
impl crate::Readable for Adc0ClkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`adc0_clkcfg::W`](W) writer structure"]
impl crate::Writable for Adc0ClkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC0_CLKCFG to value 0"]
impl crate::Resettable for Adc0ClkcfgSpec {}
