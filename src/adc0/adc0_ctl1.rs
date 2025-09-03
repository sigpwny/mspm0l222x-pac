#[doc = "Register `ADC0_CTL1` reader"]
pub type R = crate::R<Adc0Ctl1Spec>;
#[doc = "Register `ADC0_CTL1` writer"]
pub type W = crate::W<Adc0Ctl1Spec>;
#[doc = "Sample trigger source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigsrc {
    #[doc = "0: Software trigger"]
    Software = 0,
    #[doc = "1: Hardware event trigger"]
    Event = 1,
}
impl From<Trigsrc> for bool {
    #[inline(always)]
    fn from(variant: Trigsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGSRC` reader - Sample trigger source"]
pub type TrigsrcR = crate::BitReader<Trigsrc>;
impl TrigsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigsrc {
        match self.bits {
            false => Trigsrc::Software,
            true => Trigsrc::Event,
        }
    }
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Trigsrc::Software
    }
    #[doc = "Hardware event trigger"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Trigsrc::Event
    }
}
#[doc = "Field `TRIGSRC` writer - Sample trigger source"]
pub type TrigsrcW<'a, REG> = crate::BitWriter<'a, REG, Trigsrc>;
impl<'a, REG> TrigsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrc::Software)
    }
    #[doc = "Hardware event trigger"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrc::Event)
    }
}
#[doc = "Start of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sc {
    #[doc = "0: When SAMPMODE is set to MANUAL, clearing this bit will end the sample phase and the conversion phase will start. When SAMPMODE is set to AUTO, writing 0 has no effect."]
    Stop = 0,
    #[doc = "1: When SAMPMODE is set to MANUAL, setting this bit will start the sample phase. Sample phase will last as long as this bit is set. When SAMPMODE is set to AUTO, setting this bit will trigger the timer based sample time."]
    Start = 1,
}
impl From<Sc> for bool {
    #[inline(always)]
    fn from(variant: Sc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC` reader - Start of conversion"]
pub type ScR = crate::BitReader<Sc>;
impl ScR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sc {
        match self.bits {
            false => Sc::Stop,
            true => Sc::Start,
        }
    }
    #[doc = "When SAMPMODE is set to MANUAL, clearing this bit will end the sample phase and the conversion phase will start. When SAMPMODE is set to AUTO, writing 0 has no effect."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Sc::Stop
    }
    #[doc = "When SAMPMODE is set to MANUAL, setting this bit will start the sample phase. Sample phase will last as long as this bit is set. When SAMPMODE is set to AUTO, setting this bit will trigger the timer based sample time."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Sc::Start
    }
}
#[doc = "Field `SC` writer - Start of conversion"]
pub type ScW<'a, REG> = crate::BitWriter<'a, REG, Sc>;
impl<'a, REG> ScW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When SAMPMODE is set to MANUAL, clearing this bit will end the sample phase and the conversion phase will start. When SAMPMODE is set to AUTO, writing 0 has no effect."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Sc::Stop)
    }
    #[doc = "When SAMPMODE is set to MANUAL, setting this bit will start the sample phase. Sample phase will last as long as this bit is set. When SAMPMODE is set to AUTO, setting this bit will trigger the timer based sample time."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Sc::Start)
    }
}
#[doc = "Conversion sequence mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Conseq {
    #[doc = "0: ADC channel in MEMCTLx pointed by STARTADD will be converted once"]
    Single = 0,
    #[doc = "1: ADC channel sequence pointed by STARTADD and ENDADD will be converted once"]
    Sequence = 1,
    #[doc = "2: ADC channel in MEMCTLx pointed by STARTADD will be converted repeatedly"]
    Repeatsingle = 2,
    #[doc = "3: ADC channel sequence pointed by STARTADD and ENDADD will be converted repeatedly"]
    Repeatsequence = 3,
}
impl From<Conseq> for u8 {
    #[inline(always)]
    fn from(variant: Conseq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Conseq {
    type Ux = u8;
}
impl crate::IsEnum for Conseq {}
#[doc = "Field `CONSEQ` reader - Conversion sequence mode"]
pub type ConseqR = crate::FieldReader<Conseq>;
impl ConseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Conseq {
        match self.bits {
            0 => Conseq::Single,
            1 => Conseq::Sequence,
            2 => Conseq::Repeatsingle,
            3 => Conseq::Repeatsequence,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC channel in MEMCTLx pointed by STARTADD will be converted once"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Conseq::Single
    }
    #[doc = "ADC channel sequence pointed by STARTADD and ENDADD will be converted once"]
    #[inline(always)]
    pub fn is_sequence(&self) -> bool {
        *self == Conseq::Sequence
    }
    #[doc = "ADC channel in MEMCTLx pointed by STARTADD will be converted repeatedly"]
    #[inline(always)]
    pub fn is_repeatsingle(&self) -> bool {
        *self == Conseq::Repeatsingle
    }
    #[doc = "ADC channel sequence pointed by STARTADD and ENDADD will be converted repeatedly"]
    #[inline(always)]
    pub fn is_repeatsequence(&self) -> bool {
        *self == Conseq::Repeatsequence
    }
}
#[doc = "Field `CONSEQ` writer - Conversion sequence mode"]
pub type ConseqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Conseq, crate::Safe>;
impl<'a, REG> ConseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC channel in MEMCTLx pointed by STARTADD will be converted once"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Single)
    }
    #[doc = "ADC channel sequence pointed by STARTADD and ENDADD will be converted once"]
    #[inline(always)]
    pub fn sequence(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Sequence)
    }
    #[doc = "ADC channel in MEMCTLx pointed by STARTADD will be converted repeatedly"]
    #[inline(always)]
    pub fn repeatsingle(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Repeatsingle)
    }
    #[doc = "ADC channel sequence pointed by STARTADD and ENDADD will be converted repeatedly"]
    #[inline(always)]
    pub fn repeatsequence(self) -> &'a mut crate::W<REG> {
        self.variant(Conseq::Repeatsequence)
    }
}
#[doc = "Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sampmode {
    #[doc = "0: Sample timer high phase is used as sample signal"]
    Auto = 0,
    #[doc = "1: Software trigger is used as sample signal"]
    Manual = 1,
}
impl From<Sampmode> for bool {
    #[inline(always)]
    fn from(variant: Sampmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPMODE` reader - Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
pub type SampmodeR = crate::BitReader<Sampmode>;
impl SampmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sampmode {
        match self.bits {
            false => Sampmode::Auto,
            true => Sampmode::Manual,
        }
    }
    #[doc = "Sample timer high phase is used as sample signal"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Sampmode::Auto
    }
    #[doc = "Software trigger is used as sample signal"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == Sampmode::Manual
    }
}
#[doc = "Field `SAMPMODE` writer - Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
pub type SampmodeW<'a, REG> = crate::BitWriter<'a, REG, Sampmode>;
impl<'a, REG> SampmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sample timer high phase is used as sample signal"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Sampmode::Auto)
    }
    #[doc = "Software trigger is used as sample signal"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(Sampmode::Manual)
    }
}
#[doc = "Hardware averager numerator. Selects number of conversions to accumulate for current MEMCTLx and then it is divided by AVGD. Result will be stored in MEMRESx.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Avgn {
    #[doc = "0: Disables averager"]
    Disable = 0,
    #[doc = "1: Averages 2 conversions before storing in MEMRESx register"]
    Avg2 = 1,
    #[doc = "2: Averages 4 conversions before storing in MEMRESx register"]
    Avg4 = 2,
    #[doc = "3: Averages 8 conversions before storing in MEMRESx register"]
    Avg8 = 3,
    #[doc = "4: Averages 16 conversions before storing in MEMRESx register"]
    Avg16 = 4,
    #[doc = "5: Averages 32 conversions before storing in MEMRESx register"]
    Avg32 = 5,
    #[doc = "6: Averages 64 conversions before storing in MEMRESx register"]
    Avg64 = 6,
    #[doc = "7: Averages 128 conversions before storing in MEMRESx register"]
    Avg128 = 7,
}
impl From<Avgn> for u8 {
    #[inline(always)]
    fn from(variant: Avgn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Avgn {
    type Ux = u8;
}
impl crate::IsEnum for Avgn {}
#[doc = "Field `AVGN` reader - Hardware averager numerator. Selects number of conversions to accumulate for current MEMCTLx and then it is divided by AVGD. Result will be stored in MEMRESx."]
pub type AvgnR = crate::FieldReader<Avgn>;
impl AvgnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avgn {
        match self.bits {
            0 => Avgn::Disable,
            1 => Avgn::Avg2,
            2 => Avgn::Avg4,
            3 => Avgn::Avg8,
            4 => Avgn::Avg16,
            5 => Avgn::Avg32,
            6 => Avgn::Avg64,
            7 => Avgn::Avg128,
            _ => unreachable!(),
        }
    }
    #[doc = "Disables averager"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Avgn::Disable
    }
    #[doc = "Averages 2 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn is_avg_2(&self) -> bool {
        *self == Avgn::Avg2
    }
    #[doc = "Averages 4 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn is_avg_4(&self) -> bool {
        *self == Avgn::Avg4
    }
    #[doc = "Averages 8 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn is_avg_8(&self) -> bool {
        *self == Avgn::Avg8
    }
    #[doc = "Averages 16 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn is_avg_16(&self) -> bool {
        *self == Avgn::Avg16
    }
    #[doc = "Averages 32 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn is_avg_32(&self) -> bool {
        *self == Avgn::Avg32
    }
    #[doc = "Averages 64 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn is_avg_64(&self) -> bool {
        *self == Avgn::Avg64
    }
    #[doc = "Averages 128 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn is_avg_128(&self) -> bool {
        *self == Avgn::Avg128
    }
}
#[doc = "Field `AVGN` writer - Hardware averager numerator. Selects number of conversions to accumulate for current MEMCTLx and then it is divided by AVGD. Result will be stored in MEMRESx."]
pub type AvgnW<'a, REG> = crate::FieldWriter<'a, REG, 3, Avgn, crate::Safe>;
impl<'a, REG> AvgnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disables averager"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Avgn::Disable)
    }
    #[doc = "Averages 2 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn avg_2(self) -> &'a mut crate::W<REG> {
        self.variant(Avgn::Avg2)
    }
    #[doc = "Averages 4 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn avg_4(self) -> &'a mut crate::W<REG> {
        self.variant(Avgn::Avg4)
    }
    #[doc = "Averages 8 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn avg_8(self) -> &'a mut crate::W<REG> {
        self.variant(Avgn::Avg8)
    }
    #[doc = "Averages 16 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn avg_16(self) -> &'a mut crate::W<REG> {
        self.variant(Avgn::Avg16)
    }
    #[doc = "Averages 32 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn avg_32(self) -> &'a mut crate::W<REG> {
        self.variant(Avgn::Avg32)
    }
    #[doc = "Averages 64 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn avg_64(self) -> &'a mut crate::W<REG> {
        self.variant(Avgn::Avg64)
    }
    #[doc = "Averages 128 conversions before storing in MEMRESx register"]
    #[inline(always)]
    pub fn avg_128(self) -> &'a mut crate::W<REG> {
        self.variant(Avgn::Avg128)
    }
}
#[doc = "Hardware averager denominator. The number to divide the accumulated value by (this is a shift). Note result register is maximum of 16-bits long so if not shifted appropriately result will be truncated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Avgd {
    #[doc = "0: No shift"]
    Shift0 = 0,
    #[doc = "1: 1 bit shift"]
    Shift1 = 1,
    #[doc = "2: 2 bit shift"]
    Shift2 = 2,
    #[doc = "3: 3 bit shift"]
    Shift3 = 3,
    #[doc = "4: 4 bit shift"]
    Shift4 = 4,
    #[doc = "5: 5 bit shift"]
    Shift5 = 5,
    #[doc = "6: 6 bit shift"]
    Shift6 = 6,
    #[doc = "7: 7 bit shift"]
    Shift7 = 7,
}
impl From<Avgd> for u8 {
    #[inline(always)]
    fn from(variant: Avgd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Avgd {
    type Ux = u8;
}
impl crate::IsEnum for Avgd {}
#[doc = "Field `AVGD` reader - Hardware averager denominator. The number to divide the accumulated value by (this is a shift). Note result register is maximum of 16-bits long so if not shifted appropriately result will be truncated."]
pub type AvgdR = crate::FieldReader<Avgd>;
impl AvgdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avgd {
        match self.bits {
            0 => Avgd::Shift0,
            1 => Avgd::Shift1,
            2 => Avgd::Shift2,
            3 => Avgd::Shift3,
            4 => Avgd::Shift4,
            5 => Avgd::Shift5,
            6 => Avgd::Shift6,
            7 => Avgd::Shift7,
            _ => unreachable!(),
        }
    }
    #[doc = "No shift"]
    #[inline(always)]
    pub fn is_shift0(&self) -> bool {
        *self == Avgd::Shift0
    }
    #[doc = "1 bit shift"]
    #[inline(always)]
    pub fn is_shift1(&self) -> bool {
        *self == Avgd::Shift1
    }
    #[doc = "2 bit shift"]
    #[inline(always)]
    pub fn is_shift2(&self) -> bool {
        *self == Avgd::Shift2
    }
    #[doc = "3 bit shift"]
    #[inline(always)]
    pub fn is_shift3(&self) -> bool {
        *self == Avgd::Shift3
    }
    #[doc = "4 bit shift"]
    #[inline(always)]
    pub fn is_shift4(&self) -> bool {
        *self == Avgd::Shift4
    }
    #[doc = "5 bit shift"]
    #[inline(always)]
    pub fn is_shift5(&self) -> bool {
        *self == Avgd::Shift5
    }
    #[doc = "6 bit shift"]
    #[inline(always)]
    pub fn is_shift6(&self) -> bool {
        *self == Avgd::Shift6
    }
    #[doc = "7 bit shift"]
    #[inline(always)]
    pub fn is_shift7(&self) -> bool {
        *self == Avgd::Shift7
    }
}
#[doc = "Field `AVGD` writer - Hardware averager denominator. The number to divide the accumulated value by (this is a shift). Note result register is maximum of 16-bits long so if not shifted appropriately result will be truncated."]
pub type AvgdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Avgd, crate::Safe>;
impl<'a, REG> AvgdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No shift"]
    #[inline(always)]
    pub fn shift0(self) -> &'a mut crate::W<REG> {
        self.variant(Avgd::Shift0)
    }
    #[doc = "1 bit shift"]
    #[inline(always)]
    pub fn shift1(self) -> &'a mut crate::W<REG> {
        self.variant(Avgd::Shift1)
    }
    #[doc = "2 bit shift"]
    #[inline(always)]
    pub fn shift2(self) -> &'a mut crate::W<REG> {
        self.variant(Avgd::Shift2)
    }
    #[doc = "3 bit shift"]
    #[inline(always)]
    pub fn shift3(self) -> &'a mut crate::W<REG> {
        self.variant(Avgd::Shift3)
    }
    #[doc = "4 bit shift"]
    #[inline(always)]
    pub fn shift4(self) -> &'a mut crate::W<REG> {
        self.variant(Avgd::Shift4)
    }
    #[doc = "5 bit shift"]
    #[inline(always)]
    pub fn shift5(self) -> &'a mut crate::W<REG> {
        self.variant(Avgd::Shift5)
    }
    #[doc = "6 bit shift"]
    #[inline(always)]
    pub fn shift6(self) -> &'a mut crate::W<REG> {
        self.variant(Avgd::Shift6)
    }
    #[doc = "7 bit shift"]
    #[inline(always)]
    pub fn shift7(self) -> &'a mut crate::W<REG> {
        self.variant(Avgd::Shift7)
    }
}
impl R {
    #[doc = "Bit 0 - Sample trigger source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TrigsrcR {
        TrigsrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Start of conversion"]
    #[inline(always)]
    pub fn sc(&self) -> ScR {
        ScR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Conversion sequence mode"]
    #[inline(always)]
    pub fn conseq(&self) -> ConseqR {
        ConseqR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
    #[inline(always)]
    pub fn sampmode(&self) -> SampmodeR {
        SampmodeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Hardware averager numerator. Selects number of conversions to accumulate for current MEMCTLx and then it is divided by AVGD. Result will be stored in MEMRESx."]
    #[inline(always)]
    pub fn avgn(&self) -> AvgnR {
        AvgnR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Hardware averager denominator. The number to divide the accumulated value by (this is a shift). Note result register is maximum of 16-bits long so if not shifted appropriately result will be truncated."]
    #[inline(always)]
    pub fn avgd(&self) -> AvgdR {
        AvgdR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sample trigger source"]
    #[inline(always)]
    pub fn trigsrc(&mut self) -> TrigsrcW<'_, Adc0Ctl1Spec> {
        TrigsrcW::new(self, 0)
    }
    #[doc = "Bit 8 - Start of conversion"]
    #[inline(always)]
    pub fn sc(&mut self) -> ScW<'_, Adc0Ctl1Spec> {
        ScW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Conversion sequence mode"]
    #[inline(always)]
    pub fn conseq(&mut self) -> ConseqW<'_, Adc0Ctl1Spec> {
        ConseqW::new(self, 16)
    }
    #[doc = "Bit 20 - Sample mode. This bit selects the source of the sampling signal. MANUAL option is not valid when TRIGSRC is selected as hardware event trigger."]
    #[inline(always)]
    pub fn sampmode(&mut self) -> SampmodeW<'_, Adc0Ctl1Spec> {
        SampmodeW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Hardware averager numerator. Selects number of conversions to accumulate for current MEMCTLx and then it is divided by AVGD. Result will be stored in MEMRESx."]
    #[inline(always)]
    pub fn avgn(&mut self) -> AvgnW<'_, Adc0Ctl1Spec> {
        AvgnW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Hardware averager denominator. The number to divide the accumulated value by (this is a shift). Note result register is maximum of 16-bits long so if not shifted appropriately result will be truncated."]
    #[inline(always)]
    pub fn avgd(&mut self) -> AvgdW<'_, Adc0Ctl1Spec> {
        AvgdW::new(self, 28)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0Ctl1Spec;
impl crate::RegisterSpec for Adc0Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_ctl1::R`](R) reader structure"]
impl crate::Readable for Adc0Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`adc0_ctl1::W`](W) writer structure"]
impl crate::Writable for Adc0Ctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC0_CTL1 to value 0"]
impl crate::Resettable for Adc0Ctl1Spec {}
