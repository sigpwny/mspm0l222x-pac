#[doc = "Register `ADC0_INT_EVENT2_IMASK` reader"]
pub type R = crate::R<Adc0IntEvent2ImaskSpec>;
#[doc = "Register `ADC0_INT_EVENT2_IMASK` writer"]
pub type W = crate::W<Adc0IntEvent2ImaskSpec>;
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg0 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg0> for bool {
    #[inline(always)]
    fn from(variant: Memresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg0R = crate::BitReader<Memresifg0>;
impl Memresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg0 {
        match self.bits {
            false => Memresifg0::Clr,
            true => Memresifg0::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg0::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg0::Set
    }
}
#[doc = "Field `MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg0W<'a, REG> = crate::BitWriter<'a, REG, Memresifg0>;
impl<'a, REG> Memresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg0::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg0::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg1 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg1> for bool {
    #[inline(always)]
    fn from(variant: Memresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG1` reader - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg1R = crate::BitReader<Memresifg1>;
impl Memresifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg1 {
        match self.bits {
            false => Memresifg1::Clr,
            true => Memresifg1::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg1::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg1::Set
    }
}
#[doc = "Field `MEMRESIFG1` writer - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg1W<'a, REG> = crate::BitWriter<'a, REG, Memresifg1>;
impl<'a, REG> Memresifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg1::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg1::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg2 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg2> for bool {
    #[inline(always)]
    fn from(variant: Memresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG2` reader - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg2R = crate::BitReader<Memresifg2>;
impl Memresifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg2 {
        match self.bits {
            false => Memresifg2::Clr,
            true => Memresifg2::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg2::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg2::Set
    }
}
#[doc = "Field `MEMRESIFG2` writer - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg2W<'a, REG> = crate::BitWriter<'a, REG, Memresifg2>;
impl<'a, REG> Memresifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg2::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg2::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg3 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg3> for bool {
    #[inline(always)]
    fn from(variant: Memresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG3` reader - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg3R = crate::BitReader<Memresifg3>;
impl Memresifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg3 {
        match self.bits {
            false => Memresifg3::Clr,
            true => Memresifg3::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg3::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg3::Set
    }
}
#[doc = "Field `MEMRESIFG3` writer - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg3W<'a, REG> = crate::BitWriter<'a, REG, Memresifg3>;
impl<'a, REG> Memresifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg3::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg3::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg4 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg4> for bool {
    #[inline(always)]
    fn from(variant: Memresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG4` reader - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg4R = crate::BitReader<Memresifg4>;
impl Memresifg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg4 {
        match self.bits {
            false => Memresifg4::Clr,
            true => Memresifg4::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg4::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg4::Set
    }
}
#[doc = "Field `MEMRESIFG4` writer - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg4W<'a, REG> = crate::BitWriter<'a, REG, Memresifg4>;
impl<'a, REG> Memresifg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg4::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg4::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg5 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg5> for bool {
    #[inline(always)]
    fn from(variant: Memresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG5` reader - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg5R = crate::BitReader<Memresifg5>;
impl Memresifg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg5 {
        match self.bits {
            false => Memresifg5::Clr,
            true => Memresifg5::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg5::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg5::Set
    }
}
#[doc = "Field `MEMRESIFG5` writer - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg5W<'a, REG> = crate::BitWriter<'a, REG, Memresifg5>;
impl<'a, REG> Memresifg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg5::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg5::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg6 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg6> for bool {
    #[inline(always)]
    fn from(variant: Memresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG6` reader - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg6R = crate::BitReader<Memresifg6>;
impl Memresifg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg6 {
        match self.bits {
            false => Memresifg6::Clr,
            true => Memresifg6::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg6::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg6::Set
    }
}
#[doc = "Field `MEMRESIFG6` writer - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg6W<'a, REG> = crate::BitWriter<'a, REG, Memresifg6>;
impl<'a, REG> Memresifg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg6::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg6::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg7 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg7> for bool {
    #[inline(always)]
    fn from(variant: Memresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG7` reader - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg7R = crate::BitReader<Memresifg7>;
impl Memresifg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg7 {
        match self.bits {
            false => Memresifg7::Clr,
            true => Memresifg7::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg7::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg7::Set
    }
}
#[doc = "Field `MEMRESIFG7` writer - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg7W<'a, REG> = crate::BitWriter<'a, REG, Memresifg7>;
impl<'a, REG> Memresifg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg7::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg7::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg8 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg8> for bool {
    #[inline(always)]
    fn from(variant: Memresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG8` reader - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg8R = crate::BitReader<Memresifg8>;
impl Memresifg8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg8 {
        match self.bits {
            false => Memresifg8::Clr,
            true => Memresifg8::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg8::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg8::Set
    }
}
#[doc = "Field `MEMRESIFG8` writer - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg8W<'a, REG> = crate::BitWriter<'a, REG, Memresifg8>;
impl<'a, REG> Memresifg8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg8::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg8::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg9 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg9> for bool {
    #[inline(always)]
    fn from(variant: Memresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG9` reader - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg9R = crate::BitReader<Memresifg9>;
impl Memresifg9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg9 {
        match self.bits {
            false => Memresifg9::Clr,
            true => Memresifg9::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg9::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg9::Set
    }
}
#[doc = "Field `MEMRESIFG9` writer - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg9W<'a, REG> = crate::BitWriter<'a, REG, Memresifg9>;
impl<'a, REG> Memresifg9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg9::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg9::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg10 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg10> for bool {
    #[inline(always)]
    fn from(variant: Memresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG10` reader - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg10R = crate::BitReader<Memresifg10>;
impl Memresifg10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg10 {
        match self.bits {
            false => Memresifg10::Clr,
            true => Memresifg10::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg10::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg10::Set
    }
}
#[doc = "Field `MEMRESIFG10` writer - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg10W<'a, REG> = crate::BitWriter<'a, REG, Memresifg10>;
impl<'a, REG> Memresifg10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg10::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg10::Set)
    }
}
#[doc = "Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg11 {
    #[doc = "0: No new data ready."]
    Clr = 0,
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
}
impl From<Memresifg11> for bool {
    #[inline(always)]
    fn from(variant: Memresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG11` reader - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg11R = crate::BitReader<Memresifg11>;
impl Memresifg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg11 {
        match self.bits {
            false => Memresifg11::Clr,
            true => Memresifg11::Set,
        }
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg11::Clr
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg11::Set
    }
}
#[doc = "Field `MEMRESIFG11` writer - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg11W<'a, REG> = crate::BitWriter<'a, REG, Memresifg11>;
impl<'a, REG> Memresifg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg11::Clr)
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg11::Set)
    }
}
impl R {
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg0(&self) -> Memresifg0R {
        Memresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg1(&self) -> Memresifg1R {
        Memresifg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg2(&self) -> Memresifg2R {
        Memresifg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg3(&self) -> Memresifg3R {
        Memresifg3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg4(&self) -> Memresifg4R {
        Memresifg4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg5(&self) -> Memresifg5R {
        Memresifg5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg6(&self) -> Memresifg6R {
        Memresifg6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg7(&self) -> Memresifg7R {
        Memresifg7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg8(&self) -> Memresifg8R {
        Memresifg8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg9(&self) -> Memresifg9R {
        Memresifg9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg10(&self) -> Memresifg10R {
        Memresifg10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg11(&self) -> Memresifg11R {
        Memresifg11R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg0(&mut self) -> Memresifg0W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg0W::new(self, 8)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg1(&mut self) -> Memresifg1W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg1W::new(self, 9)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg2(&mut self) -> Memresifg2W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg2W::new(self, 10)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg3(&mut self) -> Memresifg3W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg3W::new(self, 11)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg4(&mut self) -> Memresifg4W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg4W::new(self, 12)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg5(&mut self) -> Memresifg5W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg5W::new(self, 13)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg6(&mut self) -> Memresifg6W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg6W::new(self, 14)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg7(&mut self) -> Memresifg7W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg7W::new(self, 15)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg8(&mut self) -> Memresifg8W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg8W::new(self, 16)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg9(&mut self) -> Memresifg9W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg9W::new(self, 17)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg10(&mut self) -> Memresifg10W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg10W::new(self, 18)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg11(&mut self) -> Memresifg11W<'_, Adc0IntEvent2ImaskSpec> {
        Memresifg11W::new(self, 19)
    }
}
#[doc = "Interrupt mask extension\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event2_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_int_event2_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adc0IntEvent2ImaskSpec;
impl crate::RegisterSpec for Adc0IntEvent2ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc0_int_event2_imask::R`](R) reader structure"]
impl crate::Readable for Adc0IntEvent2ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`adc0_int_event2_imask::W`](W) writer structure"]
impl crate::Writable for Adc0IntEvent2ImaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADC0_INT_EVENT2_IMASK to value 0"]
impl crate::Resettable for Adc0IntEvent2ImaskSpec {}
