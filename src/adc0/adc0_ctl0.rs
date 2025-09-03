#[doc = "Register `ADC0_CTL0` reader"]
pub type R = crate::R<Adc0Ctl0Spec>;
#[doc = "Register `ADC0_CTL0` writer"]
pub type W = crate::W<Adc0Ctl0Spec>;
#[doc = "Enable conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enc {
    #[doc = "0: Conversion disabled. ENC change from ON to OFF will abort single or repeat sequence on a MEMCTLx boundary. The current conversion will finish and result stored in corresponding MEMRESx."]
    Off = 0,
    #[doc = "1: Conversion enabled. ADC sequencer waits for valid trigger (software or hardware)."]
    On = 1,
}
impl From<Enc> for bool {
    #[inline(always)]
    fn from(variant: Enc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENC` reader - Enable conversion"]
pub type EncR = crate::BitReader<Enc>;
impl EncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enc {
        match self.bits {
            false => Enc::Off,
            true => Enc::On,
        }
    }
    #[doc = "Conversion disabled. ENC change from ON to OFF will abort single or repeat sequence on a MEMCTLx boundary. The current conversion will finish and result stored in corresponding MEMRESx."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Enc::Off
    }
    #[doc = "Conversion enabled. ADC sequencer waits for valid trigger (software or hardware)."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Enc::On
    }
}
#[doc = "Field `ENC` writer - Enable conversion"]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG, Enc>;
impl<'a, REG> EncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion disabled. ENC change from ON to OFF will abort single or repeat sequence on a MEMCTLx boundary. The current conversion will finish and result stored in corresponding MEMRESx."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Enc::Off)
    }
    #[doc = "Conversion enabled. ADC sequencer waits for valid trigger (software or hardware)."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Enc::On)
    }
}
#[doc = "Power down policy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrdn {
    #[doc = "0: ADC is powered down on completion of a conversion if there is no pending trigger"]
    Auto = 0,
    #[doc = "1: ADC remains powered on as long as it is enabled through software."]
    Manual = 1,
}
impl From<Pwrdn> for bool {
    #[inline(always)]
    fn from(variant: Pwrdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRDN` reader - Power down policy"]
pub type PwrdnR = crate::BitReader<Pwrdn>;
impl PwrdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrdn {
        match self.bits {
            false => Pwrdn::Auto,
            true => Pwrdn::Manual,
        }
    }
    #[doc = "ADC is powered down on completion of a conversion if there is no pending trigger"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Pwrdn::Auto
    }
    #[doc = "ADC remains powered on as long as it is enabled through software."]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Pwrdn::Manual
    }
}
#[doc = "Field `PWRDN` writer - Power down policy"]
pub type PwrdnW<'a, REG> = crate::BitWriter<'a, REG, Pwrdn>;
impl<'a, REG> PwrdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC is powered down on completion of a conversion if there is no pending trigger"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdn::Auto)
    }
    #[doc = "ADC remains powered on as long as it is enabled through software."]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrdn::Manual)
    }
}
#[doc = "Sample clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sclkdiv {
    #[doc = "0: Do not divide clock source"]
    DivBy1 = 0,
    #[doc = "1: Divide clock source by 2"]
    DivBy2 = 1,
    #[doc = "2: Divide clock source by 4"]
    DivBy4 = 2,
    #[doc = "3: Divide clock source by 8"]
    DivBy8 = 3,
    #[doc = "4: Divide clock source by 16"]
    DivBy16 = 4,
    #[doc = "5: Divide clock source by 24"]
    DivBy24 = 5,
    #[doc = "6: Divide clock source by 32"]
    DivBy32 = 6,
    #[doc = "7: Divide clock source by 48"]
    DivBy48 = 7,
}
impl From<Sclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Sclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Sclkdiv {}
#[doc = "Field `SCLKDIV` reader - Sample clock divider"]
pub type SclkdivR = crate::FieldReader<Sclkdiv>;
impl SclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sclkdiv {
        match self.bits {
            0 => Sclkdiv::DivBy1,
            1 => Sclkdiv::DivBy2,
            2 => Sclkdiv::DivBy4,
            3 => Sclkdiv::DivBy8,
            4 => Sclkdiv::DivBy16,
            5 => Sclkdiv::DivBy24,
            6 => Sclkdiv::DivBy32,
            7 => Sclkdiv::DivBy48,
            _ => unreachable!(),
        }
    }
    #[doc = "Do not divide clock source"]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == Sclkdiv::DivBy1
    }
    #[doc = "Divide clock source by 2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == Sclkdiv::DivBy2
    }
    #[doc = "Divide clock source by 4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == Sclkdiv::DivBy4
    }
    #[doc = "Divide clock source by 8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == Sclkdiv::DivBy8
    }
    #[doc = "Divide clock source by 16"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == Sclkdiv::DivBy16
    }
    #[doc = "Divide clock source by 24"]
    #[inline(always)]
    pub fn is_div_by_24(&self) -> bool {
        *self == Sclkdiv::DivBy24
    }
    #[doc = "Divide clock source by 32"]
    #[inline(always)]
    pub fn is_div_by_32(&self) -> bool {
        *self == Sclkdiv::DivBy32
    }
    #[doc = "Divide clock source by 48"]
    #[inline(always)]
    pub fn is_div_by_48(&self) -> bool {
        *self == Sclkdiv::DivBy48
    }
}
#[doc = "Field `SCLKDIV` writer - Sample clock divider"]
pub type SclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sclkdiv, crate::Safe>;
impl<'a, REG> SclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not divide clock source"]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy1)
    }
    #[doc = "Divide clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy2)
    }
    #[doc = "Divide clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy4)
    }
    #[doc = "Divide clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy8)
    }
    #[doc = "Divide clock source by 16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy16)
    }
    #[doc = "Divide clock source by 24"]
    #[inline(always)]
    pub fn div_by_24(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy24)
    }
    #[doc = "Divide clock source by 32"]
    #[inline(always)]
    pub fn div_by_32(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy32)
    }
    #[doc = "Divide clock source by 48"]
    #[inline(always)]
    pub fn div_by_48(self) -> &'a mut crate::W<REG> {
        self.variant(Sclkdiv::DivBy48)
    }
}
impl R {
    #[doc = "Bit 0 - Enable conversion"]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Power down policy"]
    #[inline(always)]
    pub fn pwrdn(&self) -> PwrdnR {
        PwrdnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Sample clock divider"]
    #[inline(always)]
    pub fn sclkdiv(&self) -> SclkdivR {
        SclkdivR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable conversion"]
    #[inline(always)]
    pub fn enc(&mut self) -> EncW<'_, Adc0Ctl0Spec> {
        EncW::new(self, 0)
    }
    #[doc = "Bit 16 - Power down policy"]
    #[inline(always)]
    pub fn pwrdn(&mut self) -> PwrdnW<'_, Adc0Ctl0Spec> {
        PwrdnW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Sample clock divider"]
    #[inline(always)]
    pub fn sclkdiv(&mut self) -> SclkdivW<'_, Adc0Ctl0Spec> {
        SclkdivW::new(self, 24)
    }
}
#[doc = "Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0Ctl0Spec;
impl crate::RegisterSpec for Adc0Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_ctl0::R`](R) reader structure"]
impl crate::Readable for Adc0Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`adc0_ctl0::W`](W) writer structure"]
impl crate::Writable for Adc0Ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC0_CTL0 to value 0"]
impl crate::Resettable for Adc0Ctl0Spec {}
