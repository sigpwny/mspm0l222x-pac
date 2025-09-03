#[doc = "Register `ADC0_INT_EVENT1_IIDX` reader"]
pub type R = crate::R<Adc0IntEvent1IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Stat {
    #[doc = "0: No bit is set means there is no pending interrupt request"]
    NoIntr = 0,
    #[doc = "3: High threshold compare interrupt"]
    Highifg = 3,
    #[doc = "4: Low threshold compare interrupt"]
    Lowifg = 4,
    #[doc = "5: Primary Sequence In range comparator interrupt"]
    Inifg = 5,
    #[doc = "9: MEMRES0 data loaded interrupt"]
    Memresifg0 = 9,
}
impl From<Stat> for u16 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u16;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - Interrupt index status"]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            0 => Some(Stat::NoIntr),
            3 => Some(Stat::Highifg),
            4 => Some(Stat::Lowifg),
            5 => Some(Stat::Inifg),
            9 => Some(Stat::Memresifg0),
            _ => None,
        }
    }
    #[doc = "No bit is set means there is no pending interrupt request"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "High threshold compare interrupt"]
    #[inline(always)]
    pub fn is_highifg(&self) -> bool {
        *self == Stat::Highifg
    }
    #[doc = "Low threshold compare interrupt"]
    #[inline(always)]
    pub fn is_lowifg(&self) -> bool {
        *self == Stat::Lowifg
    }
    #[doc = "Primary Sequence In range comparator interrupt"]
    #[inline(always)]
    pub fn is_inifg(&self) -> bool {
        *self == Stat::Inifg
    }
    #[doc = "MEMRES0 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg0(&self) -> bool {
        *self == Stat::Memresifg0
    }
}
impl R {
    #[doc = "Bits 0:9 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event1_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0IntEvent1IidxSpec;
impl crate::RegisterSpec for Adc0IntEvent1IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_int_event1_iidx::R`](R) reader structure"]
impl crate::Readable for Adc0IntEvent1IidxSpec {}
#[doc = "`reset()` method sets ADC0_INT_EVENT1_IIDX to value 0"]
impl crate::Resettable for Adc0IntEvent1IidxSpec {}
