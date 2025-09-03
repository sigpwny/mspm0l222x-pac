#[doc = "Register `ADC0_CLKFREQ` reader"]
pub type R = crate::R<Adc0ClkfreqSpec>;
#[doc = "Register `ADC0_CLKFREQ` writer"]
pub type W = crate::W<Adc0ClkfreqSpec>;
#[doc = "Frequency Range.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frange {
    #[doc = "0: 1 to 4 MHz"]
    Range1to4 = 0,
    #[doc = "1: >4 to 8 MHz"]
    Range4to8 = 1,
    #[doc = "2: >8 to 16 MHz"]
    Range8to16 = 2,
    #[doc = "3: >16 to 20 MHz"]
    Range16to20 = 3,
    #[doc = "4: >20 to 24 MHz"]
    Range20to24 = 4,
    #[doc = "5: >24 to 32 MHz"]
    Range24to32 = 5,
    #[doc = "6: >32 to 40 MHz"]
    Range32to40 = 6,
    #[doc = "7: >40 to 48 MHz"]
    Range40to48 = 7,
}
impl From<Frange> for u8 {
    #[inline(always)]
    fn from(variant: Frange) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frange {
    type Ux = u8;
}
impl crate::IsEnum for Frange {}
#[doc = "Field `FRANGE` reader - Frequency Range."]
pub type FrangeR = crate::FieldReader<Frange>;
impl FrangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frange {
        match self.bits {
            0 => Frange::Range1to4,
            1 => Frange::Range4to8,
            2 => Frange::Range8to16,
            3 => Frange::Range16to20,
            4 => Frange::Range20to24,
            5 => Frange::Range24to32,
            6 => Frange::Range32to40,
            7 => Frange::Range40to48,
            _ => unreachable!(),
        }
    }
    #[doc = "1 to 4 MHz"]
    #[inline(always)]
    pub fn is_range1to4(&self) -> bool {
        *self == Frange::Range1to4
    }
    #[doc = ">4 to 8 MHz"]
    #[inline(always)]
    pub fn is_range4to8(&self) -> bool {
        *self == Frange::Range4to8
    }
    #[doc = ">8 to 16 MHz"]
    #[inline(always)]
    pub fn is_range8to16(&self) -> bool {
        *self == Frange::Range8to16
    }
    #[doc = ">16 to 20 MHz"]
    #[inline(always)]
    pub fn is_range16to20(&self) -> bool {
        *self == Frange::Range16to20
    }
    #[doc = ">20 to 24 MHz"]
    #[inline(always)]
    pub fn is_range20to24(&self) -> bool {
        *self == Frange::Range20to24
    }
    #[doc = ">24 to 32 MHz"]
    #[inline(always)]
    pub fn is_range24to32(&self) -> bool {
        *self == Frange::Range24to32
    }
    #[doc = ">32 to 40 MHz"]
    #[inline(always)]
    pub fn is_range32to40(&self) -> bool {
        *self == Frange::Range32to40
    }
    #[doc = ">40 to 48 MHz"]
    #[inline(always)]
    pub fn is_range40to48(&self) -> bool {
        *self == Frange::Range40to48
    }
}
#[doc = "Field `FRANGE` writer - Frequency Range."]
pub type FrangeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Frange, crate::Safe>;
impl<'a, REG> FrangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 to 4 MHz"]
    #[inline(always)]
    pub fn range1to4(self) -> &'a mut crate::W<REG> {
        self.variant(Frange::Range1to4)
    }
    #[doc = ">4 to 8 MHz"]
    #[inline(always)]
    pub fn range4to8(self) -> &'a mut crate::W<REG> {
        self.variant(Frange::Range4to8)
    }
    #[doc = ">8 to 16 MHz"]
    #[inline(always)]
    pub fn range8to16(self) -> &'a mut crate::W<REG> {
        self.variant(Frange::Range8to16)
    }
    #[doc = ">16 to 20 MHz"]
    #[inline(always)]
    pub fn range16to20(self) -> &'a mut crate::W<REG> {
        self.variant(Frange::Range16to20)
    }
    #[doc = ">20 to 24 MHz"]
    #[inline(always)]
    pub fn range20to24(self) -> &'a mut crate::W<REG> {
        self.variant(Frange::Range20to24)
    }
    #[doc = ">24 to 32 MHz"]
    #[inline(always)]
    pub fn range24to32(self) -> &'a mut crate::W<REG> {
        self.variant(Frange::Range24to32)
    }
    #[doc = ">32 to 40 MHz"]
    #[inline(always)]
    pub fn range32to40(self) -> &'a mut crate::W<REG> {
        self.variant(Frange::Range32to40)
    }
    #[doc = ">40 to 48 MHz"]
    #[inline(always)]
    pub fn range40to48(self) -> &'a mut crate::W<REG> {
        self.variant(Frange::Range40to48)
    }
}
impl R {
    #[doc = "Bits 0:2 - Frequency Range."]
    #[inline(always)]
    pub fn frange(&self) -> FrangeR {
        FrangeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Frequency Range."]
    #[inline(always)]
    pub fn frange(&mut self) -> FrangeW<'_, Adc0ClkfreqSpec> {
        FrangeW::new(self, 0)
    }
}
#[doc = "Sample Clock Frequency Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_clkfreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_clkfreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0ClkfreqSpec;
impl crate::RegisterSpec for Adc0ClkfreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_clkfreq::R`](R) reader structure"]
impl crate::Readable for Adc0ClkfreqSpec {}
#[doc = "`write(|w| ..)` method takes [`adc0_clkfreq::W`](W) writer structure"]
impl crate::Writable for Adc0ClkfreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC0_CLKFREQ to value 0"]
impl crate::Resettable for Adc0ClkfreqSpec {}
