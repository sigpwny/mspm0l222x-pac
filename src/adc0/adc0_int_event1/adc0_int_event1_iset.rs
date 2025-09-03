#[doc = "Register `ADC0_INT_EVENT1_ISET` writer"]
pub type W = crate::W<Adc0IntEvent1IsetSpec>;
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Highifg {
    #[doc = "0: Interrupt is not pending."]
    NoEffect = 0,
    #[doc = "1: Interrupt is pending."]
    Set = 1,
}
impl From<Highifg> for bool {
    #[inline(always)]
    fn from(variant: Highifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGHIFG` writer - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type HighifgW<'a, REG> = crate::BitWriter<'a, REG, Highifg>;
impl<'a, REG> HighifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Highifg::NoEffect)
    }
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Highifg::Set)
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lowifg {
    #[doc = "0: Interrupt is not pending."]
    NoEffect = 0,
    #[doc = "1: Interrupt is pending."]
    Set = 1,
}
impl From<Lowifg> for bool {
    #[inline(always)]
    fn from(variant: Lowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWIFG` writer - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type LowifgW<'a, REG> = crate::BitWriter<'a, REG, Lowifg>;
impl<'a, REG> LowifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Lowifg::NoEffect)
    }
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Lowifg::Set)
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inifg {
    #[doc = "0: Interrupt is not pending."]
    NoEffect = 0,
    #[doc = "1: Interrupt is pending."]
    Set = 1,
}
impl From<Inifg> for bool {
    #[inline(always)]
    fn from(variant: Inifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIFG` writer - Mask INIFG in MIS_EX register."]
pub type InifgW<'a, REG> = crate::BitWriter<'a, REG, Inifg>;
impl<'a, REG> InifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Inifg::NoEffect)
    }
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Inifg::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg0 {
    #[doc = "0: No new data ready."]
    NoEffect = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg0> for bool {
    #[inline(always)]
    fn from(variant: Memresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg0W<'a, REG> = crate::BitWriter<'a, REG, Memresifg0>;
impl<'a, REG> Memresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg0::NoEffect)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg0::Set)
    }
}
impl W {
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn highifg(&mut self) -> HighifgW<'_, Adc0IntEvent1IsetSpec> {
        HighifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn lowifg(&mut self) -> LowifgW<'_, Adc0IntEvent1IsetSpec> {
        LowifgW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn inifg(&mut self) -> InifgW<'_, Adc0IntEvent1IsetSpec> {
        InifgW::new(self, 4)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg0(&mut self) -> Memresifg0W<'_, Adc0IntEvent1IsetSpec> {
        Memresifg0W::new(self, 8)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_int_event1_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0IntEvent1IsetSpec;
impl crate::RegisterSpec for Adc0IntEvent1IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`adc0_int_event1_iset::W`](W) writer structure"]
impl crate::Writable for Adc0IntEvent1IsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC0_INT_EVENT1_ISET to value 0"]
impl crate::Resettable for Adc0IntEvent1IsetSpec {}
