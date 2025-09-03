#[doc = "Register `ADC0_WCLOW` reader"]
pub type R = crate::R<Adc0WclowSpec>;
#[doc = "Register `ADC0_WCLOW` writer"]
pub type W = crate::W<Adc0WclowSpec>;
#[doc = "Field `DATA` reader - If DF = 0, unsigned binary format has to be used. The value based on the resolution has to be right aligned with the MSB on the left. For 10-bits and 8-bits resolution, unused bits have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bits have to be 0s."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - If DF = 0, unsigned binary format has to be used. The value based on the resolution has to be right aligned with the MSB on the left. For 10-bits and 8-bits resolution, unused bits have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bits have to be 0s."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - If DF = 0, unsigned binary format has to be used. The value based on the resolution has to be right aligned with the MSB on the left. For 10-bits and 8-bits resolution, unused bits have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bits have to be 0s."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If DF = 0, unsigned binary format has to be used. The value based on the resolution has to be right aligned with the MSB on the left. For 10-bits and 8-bits resolution, unused bits have to be 0s. If DF = 1, 2s-complement format has to be used. The value based on the resolution has to be left aligned with the LSB on the right. For 10-bits and 8-bits resolution, unused bits have to be 0s."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Adc0WclowSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Window Comparator Low Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_wclow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_wclow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0WclowSpec;
impl crate::RegisterSpec for Adc0WclowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_wclow::R`](R) reader structure"]
impl crate::Readable for Adc0WclowSpec {}
#[doc = "`write(|w| ..)` method takes [`adc0_wclow::W`](W) writer structure"]
impl crate::Writable for Adc0WclowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC0_WCLOW to value 0"]
impl crate::Resettable for Adc0WclowSpec {}
