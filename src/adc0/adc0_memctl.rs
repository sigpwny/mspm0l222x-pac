#[doc = "Register `ADC0_MEMCTL[%s]` reader"]
pub type R = crate::R<Adc0MemctlSpec>;
#[doc = "Register `ADC0_MEMCTL[%s]` writer"]
pub type W = crate::W<Adc0MemctlSpec>;
#[doc = "Input channel select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chansel {
    #[doc = "0: Selects channel 0"]
    Chan0 = 0,
    #[doc = "1: Selects channel 1"]
    Chan1 = 1,
    #[doc = "2: Selects channel 2"]
    Chan2 = 2,
    #[doc = "3: Selects channel 3"]
    Chan3 = 3,
    #[doc = "4: Selects channel 4"]
    Chan4 = 4,
    #[doc = "5: Selects channel 5"]
    Chan5 = 5,
    #[doc = "6: Selects channel 6"]
    Chan6 = 6,
    #[doc = "7: Selects channel 7"]
    Chan7 = 7,
    #[doc = "8: Selects channel 8"]
    Chan8 = 8,
    #[doc = "9: Selects channel 9"]
    Chan9 = 9,
    #[doc = "10: Selects channel 10"]
    Chan10 = 10,
    #[doc = "11: Selects channel 11"]
    Chan11 = 11,
    #[doc = "12: Selects channel 12"]
    Chan12 = 12,
    #[doc = "13: Selects channel 13"]
    Chan13 = 13,
    #[doc = "14: Selects channel 14"]
    Chan14 = 14,
    #[doc = "15: Selects channel 15"]
    Chan15 = 15,
    #[doc = "16: Selects channel 16"]
    Chan16 = 16,
    #[doc = "17: Selects channel 17"]
    Chan17 = 17,
    #[doc = "18: Selects channel 18"]
    Chan18 = 18,
    #[doc = "19: Selects channel 19"]
    Chan19 = 19,
    #[doc = "20: Selects channel 20"]
    Chan20 = 20,
    #[doc = "21: Selects channel 21"]
    Chan21 = 21,
    #[doc = "22: Selects channel 22"]
    Chan22 = 22,
    #[doc = "23: Selects channel 23"]
    Chan23 = 23,
    #[doc = "24: Selects channel 24"]
    Chan24 = 24,
    #[doc = "25: Selects channel 25"]
    Chan25 = 25,
    #[doc = "26: Selects channel 26"]
    Chan26 = 26,
    #[doc = "27: Selects channel 27"]
    Chan27 = 27,
    #[doc = "28: Selects channel 28"]
    Chan28 = 28,
    #[doc = "29: Selects channel 29"]
    Chan29 = 29,
    #[doc = "30: Selects channel 30"]
    Chan30 = 30,
    #[doc = "31: Selects channel 31"]
    Chan31 = 31,
}
impl From<Chansel> for u8 {
    #[inline(always)]
    fn from(variant: Chansel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chansel {
    type Ux = u8;
}
impl crate::IsEnum for Chansel {}
#[doc = "Field `CHANSEL` reader - Input channel select."]
pub type ChanselR = crate::FieldReader<Chansel>;
impl ChanselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chansel {
        match self.bits {
            0 => Chansel::Chan0,
            1 => Chansel::Chan1,
            2 => Chansel::Chan2,
            3 => Chansel::Chan3,
            4 => Chansel::Chan4,
            5 => Chansel::Chan5,
            6 => Chansel::Chan6,
            7 => Chansel::Chan7,
            8 => Chansel::Chan8,
            9 => Chansel::Chan9,
            10 => Chansel::Chan10,
            11 => Chansel::Chan11,
            12 => Chansel::Chan12,
            13 => Chansel::Chan13,
            14 => Chansel::Chan14,
            15 => Chansel::Chan15,
            16 => Chansel::Chan16,
            17 => Chansel::Chan17,
            18 => Chansel::Chan18,
            19 => Chansel::Chan19,
            20 => Chansel::Chan20,
            21 => Chansel::Chan21,
            22 => Chansel::Chan22,
            23 => Chansel::Chan23,
            24 => Chansel::Chan24,
            25 => Chansel::Chan25,
            26 => Chansel::Chan26,
            27 => Chansel::Chan27,
            28 => Chansel::Chan28,
            29 => Chansel::Chan29,
            30 => Chansel::Chan30,
            31 => Chansel::Chan31,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects channel 0"]
    #[inline(always)]
    pub fn is_chan_0(&self) -> bool {
        *self == Chansel::Chan0
    }
    #[doc = "Selects channel 1"]
    #[inline(always)]
    pub fn is_chan_1(&self) -> bool {
        *self == Chansel::Chan1
    }
    #[doc = "Selects channel 2"]
    #[inline(always)]
    pub fn is_chan_2(&self) -> bool {
        *self == Chansel::Chan2
    }
    #[doc = "Selects channel 3"]
    #[inline(always)]
    pub fn is_chan_3(&self) -> bool {
        *self == Chansel::Chan3
    }
    #[doc = "Selects channel 4"]
    #[inline(always)]
    pub fn is_chan_4(&self) -> bool {
        *self == Chansel::Chan4
    }
    #[doc = "Selects channel 5"]
    #[inline(always)]
    pub fn is_chan_5(&self) -> bool {
        *self == Chansel::Chan5
    }
    #[doc = "Selects channel 6"]
    #[inline(always)]
    pub fn is_chan_6(&self) -> bool {
        *self == Chansel::Chan6
    }
    #[doc = "Selects channel 7"]
    #[inline(always)]
    pub fn is_chan_7(&self) -> bool {
        *self == Chansel::Chan7
    }
    #[doc = "Selects channel 8"]
    #[inline(always)]
    pub fn is_chan_8(&self) -> bool {
        *self == Chansel::Chan8
    }
    #[doc = "Selects channel 9"]
    #[inline(always)]
    pub fn is_chan_9(&self) -> bool {
        *self == Chansel::Chan9
    }
    #[doc = "Selects channel 10"]
    #[inline(always)]
    pub fn is_chan_10(&self) -> bool {
        *self == Chansel::Chan10
    }
    #[doc = "Selects channel 11"]
    #[inline(always)]
    pub fn is_chan_11(&self) -> bool {
        *self == Chansel::Chan11
    }
    #[doc = "Selects channel 12"]
    #[inline(always)]
    pub fn is_chan_12(&self) -> bool {
        *self == Chansel::Chan12
    }
    #[doc = "Selects channel 13"]
    #[inline(always)]
    pub fn is_chan_13(&self) -> bool {
        *self == Chansel::Chan13
    }
    #[doc = "Selects channel 14"]
    #[inline(always)]
    pub fn is_chan_14(&self) -> bool {
        *self == Chansel::Chan14
    }
    #[doc = "Selects channel 15"]
    #[inline(always)]
    pub fn is_chan_15(&self) -> bool {
        *self == Chansel::Chan15
    }
    #[doc = "Selects channel 16"]
    #[inline(always)]
    pub fn is_chan_16(&self) -> bool {
        *self == Chansel::Chan16
    }
    #[doc = "Selects channel 17"]
    #[inline(always)]
    pub fn is_chan_17(&self) -> bool {
        *self == Chansel::Chan17
    }
    #[doc = "Selects channel 18"]
    #[inline(always)]
    pub fn is_chan_18(&self) -> bool {
        *self == Chansel::Chan18
    }
    #[doc = "Selects channel 19"]
    #[inline(always)]
    pub fn is_chan_19(&self) -> bool {
        *self == Chansel::Chan19
    }
    #[doc = "Selects channel 20"]
    #[inline(always)]
    pub fn is_chan_20(&self) -> bool {
        *self == Chansel::Chan20
    }
    #[doc = "Selects channel 21"]
    #[inline(always)]
    pub fn is_chan_21(&self) -> bool {
        *self == Chansel::Chan21
    }
    #[doc = "Selects channel 22"]
    #[inline(always)]
    pub fn is_chan_22(&self) -> bool {
        *self == Chansel::Chan22
    }
    #[doc = "Selects channel 23"]
    #[inline(always)]
    pub fn is_chan_23(&self) -> bool {
        *self == Chansel::Chan23
    }
    #[doc = "Selects channel 24"]
    #[inline(always)]
    pub fn is_chan_24(&self) -> bool {
        *self == Chansel::Chan24
    }
    #[doc = "Selects channel 25"]
    #[inline(always)]
    pub fn is_chan_25(&self) -> bool {
        *self == Chansel::Chan25
    }
    #[doc = "Selects channel 26"]
    #[inline(always)]
    pub fn is_chan_26(&self) -> bool {
        *self == Chansel::Chan26
    }
    #[doc = "Selects channel 27"]
    #[inline(always)]
    pub fn is_chan_27(&self) -> bool {
        *self == Chansel::Chan27
    }
    #[doc = "Selects channel 28"]
    #[inline(always)]
    pub fn is_chan_28(&self) -> bool {
        *self == Chansel::Chan28
    }
    #[doc = "Selects channel 29"]
    #[inline(always)]
    pub fn is_chan_29(&self) -> bool {
        *self == Chansel::Chan29
    }
    #[doc = "Selects channel 30"]
    #[inline(always)]
    pub fn is_chan_30(&self) -> bool {
        *self == Chansel::Chan30
    }
    #[doc = "Selects channel 31"]
    #[inline(always)]
    pub fn is_chan_31(&self) -> bool {
        *self == Chansel::Chan31
    }
}
#[doc = "Field `CHANSEL` writer - Input channel select."]
pub type ChanselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Chansel, crate::Safe>;
impl<'a, REG> ChanselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects channel 0"]
    #[inline(always)]
    pub fn chan_0(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan0)
    }
    #[doc = "Selects channel 1"]
    #[inline(always)]
    pub fn chan_1(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan1)
    }
    #[doc = "Selects channel 2"]
    #[inline(always)]
    pub fn chan_2(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan2)
    }
    #[doc = "Selects channel 3"]
    #[inline(always)]
    pub fn chan_3(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan3)
    }
    #[doc = "Selects channel 4"]
    #[inline(always)]
    pub fn chan_4(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan4)
    }
    #[doc = "Selects channel 5"]
    #[inline(always)]
    pub fn chan_5(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan5)
    }
    #[doc = "Selects channel 6"]
    #[inline(always)]
    pub fn chan_6(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan6)
    }
    #[doc = "Selects channel 7"]
    #[inline(always)]
    pub fn chan_7(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan7)
    }
    #[doc = "Selects channel 8"]
    #[inline(always)]
    pub fn chan_8(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan8)
    }
    #[doc = "Selects channel 9"]
    #[inline(always)]
    pub fn chan_9(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan9)
    }
    #[doc = "Selects channel 10"]
    #[inline(always)]
    pub fn chan_10(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan10)
    }
    #[doc = "Selects channel 11"]
    #[inline(always)]
    pub fn chan_11(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan11)
    }
    #[doc = "Selects channel 12"]
    #[inline(always)]
    pub fn chan_12(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan12)
    }
    #[doc = "Selects channel 13"]
    #[inline(always)]
    pub fn chan_13(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan13)
    }
    #[doc = "Selects channel 14"]
    #[inline(always)]
    pub fn chan_14(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan14)
    }
    #[doc = "Selects channel 15"]
    #[inline(always)]
    pub fn chan_15(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan15)
    }
    #[doc = "Selects channel 16"]
    #[inline(always)]
    pub fn chan_16(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan16)
    }
    #[doc = "Selects channel 17"]
    #[inline(always)]
    pub fn chan_17(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan17)
    }
    #[doc = "Selects channel 18"]
    #[inline(always)]
    pub fn chan_18(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan18)
    }
    #[doc = "Selects channel 19"]
    #[inline(always)]
    pub fn chan_19(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan19)
    }
    #[doc = "Selects channel 20"]
    #[inline(always)]
    pub fn chan_20(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan20)
    }
    #[doc = "Selects channel 21"]
    #[inline(always)]
    pub fn chan_21(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan21)
    }
    #[doc = "Selects channel 22"]
    #[inline(always)]
    pub fn chan_22(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan22)
    }
    #[doc = "Selects channel 23"]
    #[inline(always)]
    pub fn chan_23(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan23)
    }
    #[doc = "Selects channel 24"]
    #[inline(always)]
    pub fn chan_24(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan24)
    }
    #[doc = "Selects channel 25"]
    #[inline(always)]
    pub fn chan_25(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan25)
    }
    #[doc = "Selects channel 26"]
    #[inline(always)]
    pub fn chan_26(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan26)
    }
    #[doc = "Selects channel 27"]
    #[inline(always)]
    pub fn chan_27(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan27)
    }
    #[doc = "Selects channel 28"]
    #[inline(always)]
    pub fn chan_28(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan28)
    }
    #[doc = "Selects channel 29"]
    #[inline(always)]
    pub fn chan_29(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan29)
    }
    #[doc = "Selects channel 30"]
    #[inline(always)]
    pub fn chan_30(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan30)
    }
    #[doc = "Selects channel 31"]
    #[inline(always)]
    pub fn chan_31(self) -> &'a mut crate::W<REG> {
        self.variant(Chansel::Chan31)
    }
}
#[doc = "Voltage reference selection. VEREFM must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vrsel {
    #[doc = "0: VDDA reference"]
    Vdda = 0,
    #[doc = "1: External reference from pin"]
    Extref = 1,
    #[doc = "2: Internal reference"]
    Intref = 2,
}
impl From<Vrsel> for u8 {
    #[inline(always)]
    fn from(variant: Vrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vrsel {
    type Ux = u8;
}
impl crate::IsEnum for Vrsel {}
#[doc = "Field `VRSEL` reader - Voltage reference selection. VEREFM must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
pub type VrselR = crate::FieldReader<Vrsel>;
impl VrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vrsel> {
        match self.bits {
            0 => Some(Vrsel::Vdda),
            1 => Some(Vrsel::Extref),
            2 => Some(Vrsel::Intref),
            _ => None,
        }
    }
    #[doc = "VDDA reference"]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == Vrsel::Vdda
    }
    #[doc = "External reference from pin"]
    #[inline(always)]
    pub fn is_extref(&self) -> bool {
        *self == Vrsel::Extref
    }
    #[doc = "Internal reference"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == Vrsel::Intref
    }
}
#[doc = "Field `VRSEL` writer - Voltage reference selection. VEREFM must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
pub type VrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vrsel>;
impl<'a, REG> VrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VDDA reference"]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut crate::W<REG> {
        self.variant(Vrsel::Vdda)
    }
    #[doc = "External reference from pin"]
    #[inline(always)]
    pub fn extref(self) -> &'a mut crate::W<REG> {
        self.variant(Vrsel::Extref)
    }
    #[doc = "Internal reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut crate::W<REG> {
        self.variant(Vrsel::Intref)
    }
}
#[doc = "Selects the source of sample timer period between SCOMP0 and SCOMP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stime {
    #[doc = "0: Select SCOMP0"]
    SelScomp0 = 0,
    #[doc = "1: Select SCOMP1"]
    SelScomp1 = 1,
}
impl From<Stime> for bool {
    #[inline(always)]
    fn from(variant: Stime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STIME` reader - Selects the source of sample timer period between SCOMP0 and SCOMP1."]
pub type StimeR = crate::BitReader<Stime>;
impl StimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stime {
        match self.bits {
            false => Stime::SelScomp0,
            true => Stime::SelScomp1,
        }
    }
    #[doc = "Select SCOMP0"]
    #[inline(always)]
    pub fn is_sel_scomp0(&self) -> bool {
        *self == Stime::SelScomp0
    }
    #[doc = "Select SCOMP1"]
    #[inline(always)]
    pub fn is_sel_scomp1(&self) -> bool {
        *self == Stime::SelScomp1
    }
}
#[doc = "Field `STIME` writer - Selects the source of sample timer period between SCOMP0 and SCOMP1."]
pub type StimeW<'a, REG> = crate::BitWriter<'a, REG, Stime>;
impl<'a, REG> StimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select SCOMP0"]
    #[inline(always)]
    pub fn sel_scomp0(self) -> &'a mut crate::W<REG> {
        self.variant(Stime::SelScomp0)
    }
    #[doc = "Select SCOMP1"]
    #[inline(always)]
    pub fn sel_scomp1(self) -> &'a mut crate::W<REG> {
        self.variant(Stime::SelScomp1)
    }
}
#[doc = "Enable hardware averaging.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avgen {
    #[doc = "0: Averaging disabled."]
    Disable = 0,
    #[doc = "1: Averaging enabled."]
    Enable = 1,
}
impl From<Avgen> for bool {
    #[inline(always)]
    fn from(variant: Avgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVGEN` reader - Enable hardware averaging."]
pub type AvgenR = crate::BitReader<Avgen>;
impl AvgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avgen {
        match self.bits {
            false => Avgen::Disable,
            true => Avgen::Enable,
        }
    }
    #[doc = "Averaging disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Avgen::Disable
    }
    #[doc = "Averaging enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Avgen::Enable
    }
}
#[doc = "Field `AVGEN` writer - Enable hardware averaging."]
pub type AvgenW<'a, REG> = crate::BitWriter<'a, REG, Avgen>;
impl<'a, REG> AvgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Averaging disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Avgen::Disable)
    }
    #[doc = "Averaging enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Avgen::Enable)
    }
}
#[doc = "Enable burn out current source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcsen {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Bcsen> for bool {
    #[inline(always)]
    fn from(variant: Bcsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCSEN` reader - Enable burn out current source."]
pub type BcsenR = crate::BitReader<Bcsen>;
impl BcsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcsen {
        match self.bits {
            false => Bcsen::Disable,
            true => Bcsen::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bcsen::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bcsen::Enable
    }
}
#[doc = "Field `BCSEN` writer - Enable burn out current source."]
pub type BcsenW<'a, REG> = crate::BitWriter<'a, REG, Bcsen>;
impl<'a, REG> BcsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bcsen::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bcsen::Enable)
    }
}
#[doc = "Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig {
    #[doc = "0: Next conversion is automatic"]
    AutoNext = 0,
    #[doc = "1: Next conversion requires a trigger"]
    TriggerNext = 1,
}
impl From<Trig> for bool {
    #[inline(always)]
    fn from(variant: Trig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG` reader - Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
pub type TrigR = crate::BitReader<Trig>;
impl TrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trig {
        match self.bits {
            false => Trig::AutoNext,
            true => Trig::TriggerNext,
        }
    }
    #[doc = "Next conversion is automatic"]
    #[inline(always)]
    pub fn is_auto_next(&self) -> bool {
        *self == Trig::AutoNext
    }
    #[doc = "Next conversion requires a trigger"]
    #[inline(always)]
    pub fn is_trigger_next(&self) -> bool {
        *self == Trig::TriggerNext
    }
}
#[doc = "Field `TRIG` writer - Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
pub type TrigW<'a, REG> = crate::BitWriter<'a, REG, Trig>;
impl<'a, REG> TrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next conversion is automatic"]
    #[inline(always)]
    pub fn auto_next(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::AutoNext)
    }
    #[doc = "Next conversion requires a trigger"]
    #[inline(always)]
    pub fn trigger_next(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::TriggerNext)
    }
}
#[doc = "Enable window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wincomp {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Wincomp> for bool {
    #[inline(always)]
    fn from(variant: Wincomp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINCOMP` reader - Enable window comparator."]
pub type WincompR = crate::BitReader<Wincomp>;
impl WincompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wincomp {
        match self.bits {
            false => Wincomp::Disable,
            true => Wincomp::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wincomp::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wincomp::Enable
    }
}
#[doc = "Field `WINCOMP` writer - Enable window comparator."]
pub type WincompW<'a, REG> = crate::BitWriter<'a, REG, Wincomp>;
impl<'a, REG> WincompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wincomp::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wincomp::Enable)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select."]
    #[inline(always)]
    pub fn chansel(&self) -> ChanselR {
        ChanselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Voltage reference selection. VEREFM must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
    #[inline(always)]
    pub fn vrsel(&self) -> VrselR {
        VrselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Selects the source of sample timer period between SCOMP0 and SCOMP1."]
    #[inline(always)]
    pub fn stime(&self) -> StimeR {
        StimeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable hardware averaging."]
    #[inline(always)]
    pub fn avgen(&self) -> AvgenR {
        AvgenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable burn out current source."]
    #[inline(always)]
    pub fn bcsen(&self) -> BcsenR {
        BcsenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable window comparator."]
    #[inline(always)]
    pub fn wincomp(&self) -> WincompR {
        WincompR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select."]
    #[inline(always)]
    pub fn chansel(&mut self) -> ChanselW<'_, Adc0MemctlSpec> {
        ChanselW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Voltage reference selection. VEREFM must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
    #[inline(always)]
    pub fn vrsel(&mut self) -> VrselW<'_, Adc0MemctlSpec> {
        VrselW::new(self, 8)
    }
    #[doc = "Bit 12 - Selects the source of sample timer period between SCOMP0 and SCOMP1."]
    #[inline(always)]
    pub fn stime(&mut self) -> StimeW<'_, Adc0MemctlSpec> {
        StimeW::new(self, 12)
    }
    #[doc = "Bit 16 - Enable hardware averaging."]
    #[inline(always)]
    pub fn avgen(&mut self) -> AvgenW<'_, Adc0MemctlSpec> {
        AvgenW::new(self, 16)
    }
    #[doc = "Bit 20 - Enable burn out current source."]
    #[inline(always)]
    pub fn bcsen(&mut self) -> BcsenW<'_, Adc0MemctlSpec> {
        BcsenW::new(self, 20)
    }
    #[doc = "Bit 24 - Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
    #[inline(always)]
    pub fn trig(&mut self) -> TrigW<'_, Adc0MemctlSpec> {
        TrigW::new(self, 24)
    }
    #[doc = "Bit 28 - Enable window comparator."]
    #[inline(always)]
    pub fn wincomp(&mut self) -> WincompW<'_, Adc0MemctlSpec> {
        WincompW::new(self, 28)
    }
}
#[doc = "Conversion Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_memctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_memctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0MemctlSpec;
impl crate::RegisterSpec for Adc0MemctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_memctl::R`](R) reader structure"]
impl crate::Readable for Adc0MemctlSpec {}
#[doc = "`write(|w| ..)` method takes [`adc0_memctl::W`](W) writer structure"]
impl crate::Writable for Adc0MemctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC0_MEMCTL[%s] to value 0"]
impl crate::Resettable for Adc0MemctlSpec {}
