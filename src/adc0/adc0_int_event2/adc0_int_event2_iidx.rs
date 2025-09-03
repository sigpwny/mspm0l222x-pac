#[doc = "Register `ADC0_INT_EVENT2_IIDX` reader"]
pub type R = crate::R<Adc0IntEvent2IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Stat {
    #[doc = "0: No bit is set means there is no pending interrupt request"]
    NoIntr = 0,
    #[doc = "9: MEMRES0 data loaded interrupt"]
    Memresifg0 = 9,
    #[doc = "10: MEMRES1 data loaded interrupt"]
    Memresifg1 = 10,
    #[doc = "11: MEMRES2 data loaded interrupt"]
    Memresifg2 = 11,
    #[doc = "12: MEMRES3 data loaded interrupt"]
    Memresifg3 = 12,
    #[doc = "13: MEMRES4 data loaded interrupt"]
    Memresifg4 = 13,
    #[doc = "14: MEMRES5 data loaded interrupt"]
    Memresifg5 = 14,
    #[doc = "15: MEMRES6 data loaded interrupt"]
    Memresifg6 = 15,
    #[doc = "16: MEMRES7 data loaded interrupt"]
    Memresifg7 = 16,
    #[doc = "17: MEMRES8 data loaded interrupt"]
    Memresifg8 = 17,
    #[doc = "18: MEMRES9 data loaded interrupt"]
    Memresifg9 = 18,
    #[doc = "19: MEMRES10 data loaded interrupt"]
    Memresifg10 = 19,
    #[doc = "20: MEMRES11 data loaded interrupt"]
    Memresifg11 = 20,
    #[doc = "21: MEMRES12 data loaded interrupt"]
    Memresifg12 = 21,
    #[doc = "22: MEMRES13 data loaded interrupt"]
    Memresifg13 = 22,
    #[doc = "23: MEMRES14 data loaded interrupt"]
    Memresifg14 = 23,
    #[doc = "24: MEMRES15 data loaded interrupt"]
    Memresifg15 = 24,
    #[doc = "25: MEMRES16 data loaded interrupt"]
    Memresifg16 = 25,
    #[doc = "26: MEMRES17 data loaded interrupt"]
    Memresifg17 = 26,
    #[doc = "27: MEMRES18 data loaded interrupt"]
    Memresifg18 = 27,
    #[doc = "28: MEMRES19 data loaded interrupt"]
    Memresifg19 = 28,
    #[doc = "29: MEMRES20 data loaded interrupt"]
    Memresifg20 = 29,
    #[doc = "30: MEMRES21 data loaded interrupt"]
    Memresifg21 = 30,
    #[doc = "31: MEMRES22 data loaded interrupt"]
    Memresifg22 = 31,
    #[doc = "32: MEMRES23 data loaded interrupt"]
    Memresifg23 = 32,
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
            9 => Some(Stat::Memresifg0),
            10 => Some(Stat::Memresifg1),
            11 => Some(Stat::Memresifg2),
            12 => Some(Stat::Memresifg3),
            13 => Some(Stat::Memresifg4),
            14 => Some(Stat::Memresifg5),
            15 => Some(Stat::Memresifg6),
            16 => Some(Stat::Memresifg7),
            17 => Some(Stat::Memresifg8),
            18 => Some(Stat::Memresifg9),
            19 => Some(Stat::Memresifg10),
            20 => Some(Stat::Memresifg11),
            21 => Some(Stat::Memresifg12),
            22 => Some(Stat::Memresifg13),
            23 => Some(Stat::Memresifg14),
            24 => Some(Stat::Memresifg15),
            25 => Some(Stat::Memresifg16),
            26 => Some(Stat::Memresifg17),
            27 => Some(Stat::Memresifg18),
            28 => Some(Stat::Memresifg19),
            29 => Some(Stat::Memresifg20),
            30 => Some(Stat::Memresifg21),
            31 => Some(Stat::Memresifg22),
            32 => Some(Stat::Memresifg23),
            _ => None,
        }
    }
    #[doc = "No bit is set means there is no pending interrupt request"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
    #[doc = "MEMRES0 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg0(&self) -> bool {
        *self == Stat::Memresifg0
    }
    #[doc = "MEMRES1 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg1(&self) -> bool {
        *self == Stat::Memresifg1
    }
    #[doc = "MEMRES2 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg2(&self) -> bool {
        *self == Stat::Memresifg2
    }
    #[doc = "MEMRES3 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg3(&self) -> bool {
        *self == Stat::Memresifg3
    }
    #[doc = "MEMRES4 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg4(&self) -> bool {
        *self == Stat::Memresifg4
    }
    #[doc = "MEMRES5 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg5(&self) -> bool {
        *self == Stat::Memresifg5
    }
    #[doc = "MEMRES6 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg6(&self) -> bool {
        *self == Stat::Memresifg6
    }
    #[doc = "MEMRES7 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg7(&self) -> bool {
        *self == Stat::Memresifg7
    }
    #[doc = "MEMRES8 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg8(&self) -> bool {
        *self == Stat::Memresifg8
    }
    #[doc = "MEMRES9 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg9(&self) -> bool {
        *self == Stat::Memresifg9
    }
    #[doc = "MEMRES10 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg10(&self) -> bool {
        *self == Stat::Memresifg10
    }
    #[doc = "MEMRES11 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg11(&self) -> bool {
        *self == Stat::Memresifg11
    }
    #[doc = "MEMRES12 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg12(&self) -> bool {
        *self == Stat::Memresifg12
    }
    #[doc = "MEMRES13 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg13(&self) -> bool {
        *self == Stat::Memresifg13
    }
    #[doc = "MEMRES14 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg14(&self) -> bool {
        *self == Stat::Memresifg14
    }
    #[doc = "MEMRES15 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg15(&self) -> bool {
        *self == Stat::Memresifg15
    }
    #[doc = "MEMRES16 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg16(&self) -> bool {
        *self == Stat::Memresifg16
    }
    #[doc = "MEMRES17 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg17(&self) -> bool {
        *self == Stat::Memresifg17
    }
    #[doc = "MEMRES18 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg18(&self) -> bool {
        *self == Stat::Memresifg18
    }
    #[doc = "MEMRES19 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg19(&self) -> bool {
        *self == Stat::Memresifg19
    }
    #[doc = "MEMRES20 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg20(&self) -> bool {
        *self == Stat::Memresifg20
    }
    #[doc = "MEMRES21 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg21(&self) -> bool {
        *self == Stat::Memresifg21
    }
    #[doc = "MEMRES22 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg22(&self) -> bool {
        *self == Stat::Memresifg22
    }
    #[doc = "MEMRES23 data loaded interrupt"]
    #[inline(always)]
    pub fn is_memresifg23(&self) -> bool {
        *self == Stat::Memresifg23
    }
}
impl R {
    #[doc = "Bits 0:9 - Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event2_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0IntEvent2IidxSpec;
impl crate::RegisterSpec for Adc0IntEvent2IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_int_event2_iidx::R`](R) reader structure"]
impl crate::Readable for Adc0IntEvent2IidxSpec {}
#[doc = "`reset()` method sets ADC0_INT_EVENT2_IIDX to value 0"]
impl crate::Resettable for Adc0IntEvent2IidxSpec {}
