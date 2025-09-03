#[doc = "Register `ADC0_INT_EVENT1_RIS` reader"]
pub type R = crate::R<Adc0IntEvent1RisSpec>;
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Highifg {
    #[doc = "0: Interrupt is not pending."]
    Clr = 0,
    #[doc = "1: Interrupt is pending."]
    Set = 1,
}
impl From<Highifg> for bool {
    #[inline(always)]
    fn from(variant: Highifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGHIFG` reader - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type HighifgR = crate::BitReader<Highifg>;
impl HighifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Highifg {
        match self.bits {
            false => Highifg::Clr,
            true => Highifg::Set,
        }
    }
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Highifg::Clr
    }
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Highifg::Set
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lowifg {
    #[doc = "0: Interrupt is not pending."]
    Clr = 0,
    #[doc = "1: Interrupt is pending."]
    Set = 1,
}
impl From<Lowifg> for bool {
    #[inline(always)]
    fn from(variant: Lowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWIFG` reader - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type LowifgR = crate::BitReader<Lowifg>;
impl LowifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lowifg {
        match self.bits {
            false => Lowifg::Clr,
            true => Lowifg::Set,
        }
    }
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Lowifg::Clr
    }
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Lowifg::Set
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inifg {
    #[doc = "0: Interrupt is not pending."]
    Clr = 0,
    #[doc = "1: Interrupt is pending."]
    Set = 1,
}
impl From<Inifg> for bool {
    #[inline(always)]
    fn from(variant: Inifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIFG` reader - Mask INIFG in MIS_EX register."]
pub type InifgR = crate::BitReader<Inifg>;
impl InifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inifg {
        match self.bits {
            false => Inifg::Clr,
            true => Inifg::Set,
        }
    }
    #[doc = "Interrupt is not pending."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Inifg::Clr
    }
    #[doc = "Interrupt is pending."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Inifg::Set
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg0 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg0> for bool {
    #[inline(always)]
    fn from(variant: Memresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg0R = crate::BitReader<Memresifg0>;
impl Memresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg0 {
        match self.bits {
            false => Memresifg0::Clr,
            true => Memresifg0::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg0::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg0::Set
    }
}
impl R {
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn highifg(&self) -> HighifgR {
        HighifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn lowifg(&self) -> LowifgR {
        LowifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn inifg(&self) -> InifgR {
        InifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg0(&self) -> Memresifg0R {
        Memresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event1_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0IntEvent1RisSpec;
impl crate::RegisterSpec for Adc0IntEvent1RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_int_event1_ris::R`](R) reader structure"]
impl crate::Readable for Adc0IntEvent1RisSpec {}
#[doc = "`reset()` method sets ADC0_INT_EVENT1_RIS to value 0"]
impl crate::Resettable for Adc0IntEvent1RisSpec {}
