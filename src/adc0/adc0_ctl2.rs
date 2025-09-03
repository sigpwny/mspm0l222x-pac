#[doc = "Register `ADC0_CTL2` reader"]
pub type R = crate::R<Adc0Ctl2Spec>;
#[doc = "Register `ADC0_CTL2` writer"]
pub type W = crate::W<Adc0Ctl2Spec>;
#[doc = "Data read-back format. Data is always stored in binary unsigned format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Df {
    #[doc = "0: Digital result reads as Binary Unsigned."]
    Unsigned = 0,
    #[doc = "1: Digital result reads Signed Binary. (2s complement), left aligned."]
    Signed = 1,
}
impl From<Df> for bool {
    #[inline(always)]
    fn from(variant: Df) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DF` reader - Data read-back format. Data is always stored in binary unsigned format."]
pub type DfR = crate::BitReader<Df>;
impl DfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Df {
        match self.bits {
            false => Df::Unsigned,
            true => Df::Signed,
        }
    }
    #[doc = "Digital result reads as Binary Unsigned."]
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        *self == Df::Unsigned
    }
    #[doc = "Digital result reads Signed Binary. (2s complement), left aligned."]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        *self == Df::Signed
    }
}
#[doc = "Field `DF` writer - Data read-back format. Data is always stored in binary unsigned format."]
pub type DfW<'a, REG> = crate::BitWriter<'a, REG, Df>;
impl<'a, REG> DfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital result reads as Binary Unsigned."]
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut crate::W<REG> {
        self.variant(Df::Unsigned)
    }
    #[doc = "Digital result reads Signed Binary. (2s complement), left aligned."]
    #[inline(always)]
    pub fn signed(self) -> &'a mut crate::W<REG> {
        self.variant(Df::Signed)
    }
}
#[doc = "Resolution. These bits define the resolution of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Res {
    #[doc = "0: 12-bits resolution"]
    Bit12 = 0,
    #[doc = "1: 10-bits resolution"]
    Bit10 = 1,
    #[doc = "2: 8-bits resolution"]
    Bit8 = 2,
}
impl From<Res> for u8 {
    #[inline(always)]
    fn from(variant: Res) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Res {
    type Ux = u8;
}
impl crate::IsEnum for Res {}
#[doc = "Field `RES` reader - Resolution. These bits define the resolution of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
pub type ResR = crate::FieldReader<Res>;
impl ResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Res> {
        match self.bits {
            0 => Some(Res::Bit12),
            1 => Some(Res::Bit10),
            2 => Some(Res::Bit8),
            _ => None,
        }
    }
    #[doc = "12-bits resolution"]
    #[inline(always)]
    pub fn is_bit_12(&self) -> bool {
        *self == Res::Bit12
    }
    #[doc = "10-bits resolution"]
    #[inline(always)]
    pub fn is_bit_10(&self) -> bool {
        *self == Res::Bit10
    }
    #[doc = "8-bits resolution"]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        *self == Res::Bit8
    }
}
#[doc = "Field `RES` writer - Resolution. These bits define the resolution of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 2, Res>;
impl<'a, REG> ResW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bits resolution"]
    #[inline(always)]
    pub fn bit_12(self) -> &'a mut crate::W<REG> {
        self.variant(Res::Bit12)
    }
    #[doc = "10-bits resolution"]
    #[inline(always)]
    pub fn bit_10(self) -> &'a mut crate::W<REG> {
        self.variant(Res::Bit10)
    }
    #[doc = "8-bits resolution"]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut crate::W<REG> {
        self.variant(Res::Bit8)
    }
}
#[doc = "Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: DMA trigger not enabled"]
    Disable = 0,
    #[doc = "1: DMA trigger enabled"]
    Enable = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::Disable,
            true => Dmaen::Enable,
        }
    }
    #[doc = "DMA trigger not enabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaen::Disable
    }
    #[doc = "DMA trigger enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmaen::Enable
    }
}
#[doc = "Field `DMAEN` writer - Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA trigger not enabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Disable)
    }
    #[doc = "DMA trigger enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Enable)
    }
}
#[doc = "Enable FIFO based operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoen {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Fifoen> for bool {
    #[inline(always)]
    fn from(variant: Fifoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEN` reader - Enable FIFO based operation"]
pub type FifoenR = crate::BitReader<Fifoen>;
impl FifoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifoen {
        match self.bits {
            false => Fifoen::Disable,
            true => Fifoen::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Fifoen::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Fifoen::Enable
    }
}
#[doc = "Field `FIFOEN` writer - Enable FIFO based operation"]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG, Fifoen>;
impl<'a, REG> FifoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoen::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoen::Enable)
    }
}
#[doc = "Number of ADC converted samples to be transferred on a DMA trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sampcnt {
    #[doc = "0: Minimum value"]
    Min = 0,
    #[doc = "24: Maximum value"]
    Max = 24,
}
impl From<Sampcnt> for u8 {
    #[inline(always)]
    fn from(variant: Sampcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sampcnt {
    type Ux = u8;
}
impl crate::IsEnum for Sampcnt {}
#[doc = "Field `SAMPCNT` reader - Number of ADC converted samples to be transferred on a DMA trigger"]
pub type SampcntR = crate::FieldReader<Sampcnt>;
impl SampcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sampcnt> {
        match self.bits {
            0 => Some(Sampcnt::Min),
            24 => Some(Sampcnt::Max),
            _ => None,
        }
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Sampcnt::Min
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Sampcnt::Max
    }
}
#[doc = "Field `SAMPCNT` writer - Number of ADC converted samples to be transferred on a DMA trigger"]
pub type SampcntW<'a, REG> = crate::FieldWriter<'a, REG, 5, Sampcnt>;
impl<'a, REG> SampcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Sampcnt::Min)
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Sampcnt::Max)
    }
}
#[doc = "Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startadd {
    #[doc = "0: MEMCTL0 is selected as start address of a sequence or for a single conversion."]
    Addr00 = 0,
    #[doc = "1: MEMCTL1 is selected as start address of a sequence or for a single conversion."]
    Addr01 = 1,
    #[doc = "2: MEMCTL2 is selected as start address of a sequence or for a single conversion."]
    Addr02 = 2,
    #[doc = "3: MEMCTL3 is selected as start address of a sequence or for a single conversion."]
    Addr03 = 3,
    #[doc = "4: MEMCTL4 is selected as start address of a sequence or for a single conversion."]
    Addr04 = 4,
    #[doc = "5: MEMCTL5 is selected as start address of a sequence or for a single conversion."]
    Addr05 = 5,
    #[doc = "6: MEMCTL6 is selected as start address of a sequence or for a single conversion."]
    Addr06 = 6,
    #[doc = "7: MEMCTL7 is selected as start address of a sequence or for a single conversion."]
    Addr07 = 7,
    #[doc = "8: MEMCTL8 is selected as start address of a sequence or for a single conversion."]
    Addr08 = 8,
    #[doc = "9: MEMCTL9 is selected as start address of a sequence or for a single conversion."]
    Addr09 = 9,
    #[doc = "10: MEMCTL10 is selected as start address of a sequence or for a single conversion."]
    Addr10 = 10,
    #[doc = "11: MEMCTL11 is selected as start address of a sequence or for a single conversion."]
    Addr11 = 11,
    #[doc = "12: MEMCTL12 is selected as start address of a sequence or for a single conversion."]
    Addr12 = 12,
    #[doc = "13: MEMCTL13 is selected as start address of a sequence or for a single conversion."]
    Addr13 = 13,
    #[doc = "14: MEMCTL14 is selected as start address of a sequence or for a single conversion."]
    Addr14 = 14,
    #[doc = "15: MEMCTL15 is selected as start address of a sequence or for a single conversion."]
    Addr15 = 15,
    #[doc = "16: MEMCTL16 is selected as start address of a sequence or for a single conversion."]
    Addr16 = 16,
    #[doc = "17: MEMCTL17 is selected as start address of a sequence or for a single conversion."]
    Addr17 = 17,
    #[doc = "18: MEMCTL18 is selected as start address of a sequence or for a single conversion."]
    Addr18 = 18,
    #[doc = "19: MEMCTL19 is selected as start address of a sequence or for a single conversion."]
    Addr19 = 19,
    #[doc = "20: MEMCTL20 is selected as start address of a sequence or for a single conversion."]
    Addr20 = 20,
    #[doc = "21: MEMCTL21 is selected as start address of a sequence or for a single conversion."]
    Addr21 = 21,
    #[doc = "22: MEMCTL22 is selected as start address of a sequence or for a single conversion."]
    Addr22 = 22,
    #[doc = "23: MEMCTL23 is selected as start address of a sequence or for a single conversion."]
    Addr23 = 23,
}
impl From<Startadd> for u8 {
    #[inline(always)]
    fn from(variant: Startadd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startadd {
    type Ux = u8;
}
impl crate::IsEnum for Startadd {}
#[doc = "Field `STARTADD` reader - Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type StartaddR = crate::FieldReader<Startadd>;
impl StartaddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Startadd> {
        match self.bits {
            0 => Some(Startadd::Addr00),
            1 => Some(Startadd::Addr01),
            2 => Some(Startadd::Addr02),
            3 => Some(Startadd::Addr03),
            4 => Some(Startadd::Addr04),
            5 => Some(Startadd::Addr05),
            6 => Some(Startadd::Addr06),
            7 => Some(Startadd::Addr07),
            8 => Some(Startadd::Addr08),
            9 => Some(Startadd::Addr09),
            10 => Some(Startadd::Addr10),
            11 => Some(Startadd::Addr11),
            12 => Some(Startadd::Addr12),
            13 => Some(Startadd::Addr13),
            14 => Some(Startadd::Addr14),
            15 => Some(Startadd::Addr15),
            16 => Some(Startadd::Addr16),
            17 => Some(Startadd::Addr17),
            18 => Some(Startadd::Addr18),
            19 => Some(Startadd::Addr19),
            20 => Some(Startadd::Addr20),
            21 => Some(Startadd::Addr21),
            22 => Some(Startadd::Addr22),
            23 => Some(Startadd::Addr23),
            _ => None,
        }
    }
    #[doc = "MEMCTL0 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_00(&self) -> bool {
        *self == Startadd::Addr00
    }
    #[doc = "MEMCTL1 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_01(&self) -> bool {
        *self == Startadd::Addr01
    }
    #[doc = "MEMCTL2 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_02(&self) -> bool {
        *self == Startadd::Addr02
    }
    #[doc = "MEMCTL3 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_03(&self) -> bool {
        *self == Startadd::Addr03
    }
    #[doc = "MEMCTL4 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_04(&self) -> bool {
        *self == Startadd::Addr04
    }
    #[doc = "MEMCTL5 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_05(&self) -> bool {
        *self == Startadd::Addr05
    }
    #[doc = "MEMCTL6 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_06(&self) -> bool {
        *self == Startadd::Addr06
    }
    #[doc = "MEMCTL7 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_07(&self) -> bool {
        *self == Startadd::Addr07
    }
    #[doc = "MEMCTL8 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_08(&self) -> bool {
        *self == Startadd::Addr08
    }
    #[doc = "MEMCTL9 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_09(&self) -> bool {
        *self == Startadd::Addr09
    }
    #[doc = "MEMCTL10 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_10(&self) -> bool {
        *self == Startadd::Addr10
    }
    #[doc = "MEMCTL11 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_11(&self) -> bool {
        *self == Startadd::Addr11
    }
    #[doc = "MEMCTL12 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_12(&self) -> bool {
        *self == Startadd::Addr12
    }
    #[doc = "MEMCTL13 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_13(&self) -> bool {
        *self == Startadd::Addr13
    }
    #[doc = "MEMCTL14 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_14(&self) -> bool {
        *self == Startadd::Addr14
    }
    #[doc = "MEMCTL15 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_15(&self) -> bool {
        *self == Startadd::Addr15
    }
    #[doc = "MEMCTL16 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_16(&self) -> bool {
        *self == Startadd::Addr16
    }
    #[doc = "MEMCTL17 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_17(&self) -> bool {
        *self == Startadd::Addr17
    }
    #[doc = "MEMCTL18 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_18(&self) -> bool {
        *self == Startadd::Addr18
    }
    #[doc = "MEMCTL19 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_19(&self) -> bool {
        *self == Startadd::Addr19
    }
    #[doc = "MEMCTL20 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_20(&self) -> bool {
        *self == Startadd::Addr20
    }
    #[doc = "MEMCTL21 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_21(&self) -> bool {
        *self == Startadd::Addr21
    }
    #[doc = "MEMCTL22 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_22(&self) -> bool {
        *self == Startadd::Addr22
    }
    #[doc = "MEMCTL23 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn is_addr_23(&self) -> bool {
        *self == Startadd::Addr23
    }
}
#[doc = "Field `STARTADD` writer - Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type StartaddW<'a, REG> = crate::FieldWriter<'a, REG, 5, Startadd>;
impl<'a, REG> StartaddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MEMCTL0 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_00(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr00)
    }
    #[doc = "MEMCTL1 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_01(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr01)
    }
    #[doc = "MEMCTL2 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_02(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr02)
    }
    #[doc = "MEMCTL3 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_03(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr03)
    }
    #[doc = "MEMCTL4 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_04(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr04)
    }
    #[doc = "MEMCTL5 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_05(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr05)
    }
    #[doc = "MEMCTL6 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_06(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr06)
    }
    #[doc = "MEMCTL7 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_07(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr07)
    }
    #[doc = "MEMCTL8 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_08(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr08)
    }
    #[doc = "MEMCTL9 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_09(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr09)
    }
    #[doc = "MEMCTL10 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_10(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr10)
    }
    #[doc = "MEMCTL11 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_11(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr11)
    }
    #[doc = "MEMCTL12 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_12(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr12)
    }
    #[doc = "MEMCTL13 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_13(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr13)
    }
    #[doc = "MEMCTL14 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_14(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr14)
    }
    #[doc = "MEMCTL15 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_15(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr15)
    }
    #[doc = "MEMCTL16 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_16(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr16)
    }
    #[doc = "MEMCTL17 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_17(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr17)
    }
    #[doc = "MEMCTL18 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_18(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr18)
    }
    #[doc = "MEMCTL19 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_19(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr19)
    }
    #[doc = "MEMCTL20 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_20(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr20)
    }
    #[doc = "MEMCTL21 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_21(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr21)
    }
    #[doc = "MEMCTL22 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_22(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr22)
    }
    #[doc = "MEMCTL23 is selected as start address of a sequence or for a single conversion."]
    #[inline(always)]
    pub fn addr_23(self) -> &'a mut crate::W<REG> {
        self.variant(Startadd::Addr23)
    }
}
#[doc = "Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Endadd {
    #[doc = "0: MEMCTL0 is selected as end address of sequence."]
    Addr00 = 0,
    #[doc = "1: MEMCTL1 is selected as end address of sequence."]
    Addr01 = 1,
    #[doc = "2: MEMCTL2 is selected as end address of sequence."]
    Addr02 = 2,
    #[doc = "3: MEMCTL3 is selected as end address of sequence."]
    Addr03 = 3,
    #[doc = "4: MEMCTL4 is selected as end address of sequence."]
    Addr04 = 4,
    #[doc = "5: MEMCTL5 is selected as end address of sequence."]
    Addr05 = 5,
    #[doc = "6: MEMCTL6 is selected as end address of sequence."]
    Addr06 = 6,
    #[doc = "7: MEMCTL7 is selected as end address of sequence."]
    Addr07 = 7,
    #[doc = "8: MEMCTL8 is selected as end address of sequence."]
    Addr08 = 8,
    #[doc = "9: MEMCTL9 is selected as end address of sequence."]
    Addr09 = 9,
    #[doc = "10: MEMCTL10 is selected as end address of sequence."]
    Addr10 = 10,
    #[doc = "11: MEMCTL11 is selected as end address of sequence."]
    Addr11 = 11,
    #[doc = "12: MEMCTL12 is selected as end address of sequence."]
    Addr12 = 12,
    #[doc = "13: MEMCTL13 is selected as end address of sequence."]
    Addr13 = 13,
    #[doc = "14: MEMCTL14 is selected as end address of sequence."]
    Addr14 = 14,
    #[doc = "15: MEMCTL15 is selected as end address of sequence."]
    Addr15 = 15,
    #[doc = "16: MEMCTL16 is selected as end address of sequence."]
    Addr16 = 16,
    #[doc = "17: MEMCTL17 is selected as end address of sequence."]
    Addr17 = 17,
    #[doc = "18: MEMCTL18 is selected as end address of sequence."]
    Addr18 = 18,
    #[doc = "19: MEMCTL19 is selected as end address of sequence."]
    Addr19 = 19,
    #[doc = "20: MEMCTL20 is selected as end address of sequence."]
    Addr20 = 20,
    #[doc = "21: MEMCTL21 is selected as end address of sequence."]
    Addr21 = 21,
    #[doc = "22: MEMCTL22 is selected as end address of sequence."]
    Addr22 = 22,
    #[doc = "23: MEMCTL23 is selected as end address of sequence."]
    Addr23 = 23,
}
impl From<Endadd> for u8 {
    #[inline(always)]
    fn from(variant: Endadd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Endadd {
    type Ux = u8;
}
impl crate::IsEnum for Endadd {}
#[doc = "Field `ENDADD` reader - Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type EndaddR = crate::FieldReader<Endadd>;
impl EndaddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Endadd> {
        match self.bits {
            0 => Some(Endadd::Addr00),
            1 => Some(Endadd::Addr01),
            2 => Some(Endadd::Addr02),
            3 => Some(Endadd::Addr03),
            4 => Some(Endadd::Addr04),
            5 => Some(Endadd::Addr05),
            6 => Some(Endadd::Addr06),
            7 => Some(Endadd::Addr07),
            8 => Some(Endadd::Addr08),
            9 => Some(Endadd::Addr09),
            10 => Some(Endadd::Addr10),
            11 => Some(Endadd::Addr11),
            12 => Some(Endadd::Addr12),
            13 => Some(Endadd::Addr13),
            14 => Some(Endadd::Addr14),
            15 => Some(Endadd::Addr15),
            16 => Some(Endadd::Addr16),
            17 => Some(Endadd::Addr17),
            18 => Some(Endadd::Addr18),
            19 => Some(Endadd::Addr19),
            20 => Some(Endadd::Addr20),
            21 => Some(Endadd::Addr21),
            22 => Some(Endadd::Addr22),
            23 => Some(Endadd::Addr23),
            _ => None,
        }
    }
    #[doc = "MEMCTL0 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_00(&self) -> bool {
        *self == Endadd::Addr00
    }
    #[doc = "MEMCTL1 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_01(&self) -> bool {
        *self == Endadd::Addr01
    }
    #[doc = "MEMCTL2 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_02(&self) -> bool {
        *self == Endadd::Addr02
    }
    #[doc = "MEMCTL3 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_03(&self) -> bool {
        *self == Endadd::Addr03
    }
    #[doc = "MEMCTL4 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_04(&self) -> bool {
        *self == Endadd::Addr04
    }
    #[doc = "MEMCTL5 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_05(&self) -> bool {
        *self == Endadd::Addr05
    }
    #[doc = "MEMCTL6 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_06(&self) -> bool {
        *self == Endadd::Addr06
    }
    #[doc = "MEMCTL7 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_07(&self) -> bool {
        *self == Endadd::Addr07
    }
    #[doc = "MEMCTL8 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_08(&self) -> bool {
        *self == Endadd::Addr08
    }
    #[doc = "MEMCTL9 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_09(&self) -> bool {
        *self == Endadd::Addr09
    }
    #[doc = "MEMCTL10 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_10(&self) -> bool {
        *self == Endadd::Addr10
    }
    #[doc = "MEMCTL11 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_11(&self) -> bool {
        *self == Endadd::Addr11
    }
    #[doc = "MEMCTL12 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_12(&self) -> bool {
        *self == Endadd::Addr12
    }
    #[doc = "MEMCTL13 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_13(&self) -> bool {
        *self == Endadd::Addr13
    }
    #[doc = "MEMCTL14 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_14(&self) -> bool {
        *self == Endadd::Addr14
    }
    #[doc = "MEMCTL15 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_15(&self) -> bool {
        *self == Endadd::Addr15
    }
    #[doc = "MEMCTL16 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_16(&self) -> bool {
        *self == Endadd::Addr16
    }
    #[doc = "MEMCTL17 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_17(&self) -> bool {
        *self == Endadd::Addr17
    }
    #[doc = "MEMCTL18 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_18(&self) -> bool {
        *self == Endadd::Addr18
    }
    #[doc = "MEMCTL19 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_19(&self) -> bool {
        *self == Endadd::Addr19
    }
    #[doc = "MEMCTL20 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_20(&self) -> bool {
        *self == Endadd::Addr20
    }
    #[doc = "MEMCTL21 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_21(&self) -> bool {
        *self == Endadd::Addr21
    }
    #[doc = "MEMCTL22 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_22(&self) -> bool {
        *self == Endadd::Addr22
    }
    #[doc = "MEMCTL23 is selected as end address of sequence."]
    #[inline(always)]
    pub fn is_addr_23(&self) -> bool {
        *self == Endadd::Addr23
    }
}
#[doc = "Field `ENDADD` writer - Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
pub type EndaddW<'a, REG> = crate::FieldWriter<'a, REG, 5, Endadd>;
impl<'a, REG> EndaddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MEMCTL0 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_00(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr00)
    }
    #[doc = "MEMCTL1 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_01(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr01)
    }
    #[doc = "MEMCTL2 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_02(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr02)
    }
    #[doc = "MEMCTL3 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_03(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr03)
    }
    #[doc = "MEMCTL4 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_04(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr04)
    }
    #[doc = "MEMCTL5 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_05(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr05)
    }
    #[doc = "MEMCTL6 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_06(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr06)
    }
    #[doc = "MEMCTL7 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_07(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr07)
    }
    #[doc = "MEMCTL8 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_08(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr08)
    }
    #[doc = "MEMCTL9 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_09(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr09)
    }
    #[doc = "MEMCTL10 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_10(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr10)
    }
    #[doc = "MEMCTL11 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_11(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr11)
    }
    #[doc = "MEMCTL12 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_12(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr12)
    }
    #[doc = "MEMCTL13 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_13(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr13)
    }
    #[doc = "MEMCTL14 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_14(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr14)
    }
    #[doc = "MEMCTL15 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_15(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr15)
    }
    #[doc = "MEMCTL16 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_16(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr16)
    }
    #[doc = "MEMCTL17 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_17(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr17)
    }
    #[doc = "MEMCTL18 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_18(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr18)
    }
    #[doc = "MEMCTL19 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_19(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr19)
    }
    #[doc = "MEMCTL20 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_20(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr20)
    }
    #[doc = "MEMCTL21 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_21(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr21)
    }
    #[doc = "MEMCTL22 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_22(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr22)
    }
    #[doc = "MEMCTL23 is selected as end address of sequence."]
    #[inline(always)]
    pub fn addr_23(self) -> &'a mut crate::W<REG> {
        self.variant(Endadd::Addr23)
    }
}
impl R {
    #[doc = "Bit 0 - Data read-back format. Data is always stored in binary unsigned format."]
    #[inline(always)]
    pub fn df(&self) -> DfR {
        DfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Resolution. These bits define the resolution of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable FIFO based operation"]
    #[inline(always)]
    pub fn fifoen(&self) -> FifoenR {
        FifoenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - Number of ADC converted samples to be transferred on a DMA trigger"]
    #[inline(always)]
    pub fn sampcnt(&self) -> SampcntR {
        SampcntR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    pub fn startadd(&self) -> StartaddR {
        StartaddR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    pub fn endadd(&self) -> EndaddR {
        EndaddR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data read-back format. Data is always stored in binary unsigned format."]
    #[inline(always)]
    pub fn df(&mut self) -> DfW<'_, Adc0Ctl2Spec> {
        DfW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Resolution. These bits define the resolution of ADC conversion result. Note : A value of 3 defaults to 12-bits resolution."]
    #[inline(always)]
    pub fn res(&mut self) -> ResW<'_, Adc0Ctl2Spec> {
        ResW::new(self, 1)
    }
    #[doc = "Bit 8 - Enable DMA trigger for data transfer. Note: DMAEN bit is cleared by hardware based on DMA done signal at the end of data transfer. Software has to re-enable DMAEN bit for ADC to generate DMA triggers."]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, Adc0Ctl2Spec> {
        DmaenW::new(self, 8)
    }
    #[doc = "Bit 10 - Enable FIFO based operation"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<'_, Adc0Ctl2Spec> {
        FifoenW::new(self, 10)
    }
    #[doc = "Bits 11:15 - Number of ADC converted samples to be transferred on a DMA trigger"]
    #[inline(always)]
    pub fn sampcnt(&mut self) -> SampcntW<'_, Adc0Ctl2Spec> {
        SampcntW::new(self, 11)
    }
    #[doc = "Bits 16:20 - Sequencer start address. These bits select which MEMCTLx is used for single conversion or as first MEMCTL for sequence mode. The value of STARTADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    pub fn startadd(&mut self) -> StartaddW<'_, Adc0Ctl2Spec> {
        StartaddW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Sequence end address. These bits select which MEMCTLx is the last one for the sequence mode. The value of ENDADD is 0x00 to 0x17, corresponding to MEMRES0 to MEMRES23."]
    #[inline(always)]
    pub fn endadd(&mut self) -> EndaddW<'_, Adc0Ctl2Spec> {
        EndaddW::new(self, 24)
    }
}
#[doc = "Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0Ctl2Spec;
impl crate::RegisterSpec for Adc0Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_ctl2::R`](R) reader structure"]
impl crate::Readable for Adc0Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`adc0_ctl2::W`](W) writer structure"]
impl crate::Writable for Adc0Ctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC0_CTL2 to value 0"]
impl crate::Resettable for Adc0Ctl2Spec {}
