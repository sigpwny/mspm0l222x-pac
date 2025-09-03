#[doc = "Register `ADC0_SCOMP0` reader"]
pub type R = crate::R<Adc0Scomp0Spec>;
#[doc = "Register `ADC0_SCOMP0` writer"]
pub type W = crate::W<Adc0Scomp0Spec>;
#[doc = "Field `VAL` reader - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL > 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL > 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL > 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL > 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, Adc0Scomp0Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Sample Time Compare 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_scomp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_scomp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0Scomp0Spec;
impl crate::RegisterSpec for Adc0Scomp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_scomp0::R`](R) reader structure"]
impl crate::Readable for Adc0Scomp0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc0_scomp0::W`](W) writer structure"]
impl crate::Writable for Adc0Scomp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC0_SCOMP0 to value 0"]
impl crate::Resettable for Adc0Scomp0Spec {}
