#[doc = "Register `ADC0_STATUS` reader"]
pub type R = crate::R<Adc0StatusSpec>;
#[doc = "Busy. This bit indicates that an active ADC sample or conversion operation is in progress.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: No ADC sampling or conversion in progress."]
    Idle = 0,
    #[doc = "1: ADC sampling or conversion is in progress."]
    Active = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy. This bit indicates that an active ADC sample or conversion operation is in progress."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Idle,
            true => Busy::Active,
        }
    }
    #[doc = "No ADC sampling or conversion in progress."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Busy::Idle
    }
    #[doc = "ADC sampling or conversion is in progress."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Busy::Active
    }
}
#[doc = "Indicates reference buffer is powered up and ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refbufrdy {
    #[doc = "0: Not ready"]
    Notready = 0,
    #[doc = "1: Ready"]
    Ready = 1,
}
impl From<Refbufrdy> for bool {
    #[inline(always)]
    fn from(variant: Refbufrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFBUFRDY` reader - Indicates reference buffer is powered up and ready."]
pub type RefbufrdyR = crate::BitReader<Refbufrdy>;
impl RefbufrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refbufrdy {
        match self.bits {
            false => Refbufrdy::Notready,
            true => Refbufrdy::Ready,
        }
    }
    #[doc = "Not ready"]
    #[inline(always)]
    pub fn is_notready(&self) -> bool {
        *self == Refbufrdy::Notready
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Refbufrdy::Ready
    }
}
impl R {
    #[doc = "Bit 0 - Busy. This bit indicates that an active ADC sample or conversion operation is in progress."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates reference buffer is powered up and ready."]
    #[inline(always)]
    pub fn refbufrdy(&self) -> RefbufrdyR {
        RefbufrdyR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0StatusSpec;
impl crate::RegisterSpec for Adc0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_status::R`](R) reader structure"]
impl crate::Readable for Adc0StatusSpec {}
#[doc = "`reset()` method sets ADC0_STATUS to value 0"]
impl crate::Resettable for Adc0StatusSpec {}
