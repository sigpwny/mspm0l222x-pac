#[repr(C)]
#[doc = "ADC0_INT_EVENT1\\[%s\\]"]
#[doc(alias = "ADC0_INT_EVENT1")]
pub struct Adc0IntEvent1 {
    adc0_int_event1_iidx: Adc0IntEvent1Iidx,
    _reserved1: [u8; 0x04],
    adc0_int_event1_imask: Adc0IntEvent1Imask,
    _reserved2: [u8; 0x04],
    adc0_int_event1_ris: Adc0IntEvent1Ris,
    _reserved3: [u8; 0x04],
    adc0_int_event1_mis: Adc0IntEvent1Mis,
    _reserved4: [u8; 0x04],
    adc0_int_event1_iset: Adc0IntEvent1Iset,
    _reserved5: [u8; 0x04],
    adc0_int_event1_iclr: Adc0IntEvent1Iclr,
}
impl Adc0IntEvent1 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn adc0_int_event1_iidx(&self) -> &Adc0IntEvent1Iidx {
        &self.adc0_int_event1_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn adc0_int_event1_imask(&self) -> &Adc0IntEvent1Imask {
        &self.adc0_int_event1_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn adc0_int_event1_ris(&self) -> &Adc0IntEvent1Ris {
        &self.adc0_int_event1_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn adc0_int_event1_mis(&self) -> &Adc0IntEvent1Mis {
        &self.adc0_int_event1_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn adc0_int_event1_iset(&self) -> &Adc0IntEvent1Iset {
        &self.adc0_int_event1_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn adc0_int_event1_iclr(&self) -> &Adc0IntEvent1Iclr {
        &self.adc0_int_event1_iclr
    }
}
#[doc = "ADC0_INT_EVENT1_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event1_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event1_iidx`] module"]
#[doc(alias = "ADC0_INT_EVENT1_IIDX")]
pub type Adc0IntEvent1Iidx = crate::Reg<adc0_int_event1_iidx::Adc0IntEvent1IidxSpec>;
#[doc = "Interrupt index"]
pub mod adc0_int_event1_iidx;
#[doc = "ADC0_INT_EVENT1_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event1_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_int_event1_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event1_imask`] module"]
#[doc(alias = "ADC0_INT_EVENT1_IMASK")]
pub type Adc0IntEvent1Imask = crate::Reg<adc0_int_event1_imask::Adc0IntEvent1ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod adc0_int_event1_imask;
#[doc = "ADC0_INT_EVENT1_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event1_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event1_ris`] module"]
#[doc(alias = "ADC0_INT_EVENT1_RIS")]
pub type Adc0IntEvent1Ris = crate::Reg<adc0_int_event1_ris::Adc0IntEvent1RisSpec>;
#[doc = "Raw interrupt status"]
pub mod adc0_int_event1_ris;
#[doc = "ADC0_INT_EVENT1_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event1_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event1_mis`] module"]
#[doc(alias = "ADC0_INT_EVENT1_MIS")]
pub type Adc0IntEvent1Mis = crate::Reg<adc0_int_event1_mis::Adc0IntEvent1MisSpec>;
#[doc = "Masked interrupt status"]
pub mod adc0_int_event1_mis;
#[doc = "ADC0_INT_EVENT1_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_int_event1_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event1_iset`] module"]
#[doc(alias = "ADC0_INT_EVENT1_ISET")]
pub type Adc0IntEvent1Iset = crate::Reg<adc0_int_event1_iset::Adc0IntEvent1IsetSpec>;
#[doc = "Interrupt set"]
pub mod adc0_int_event1_iset;
#[doc = "ADC0_INT_EVENT1_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_int_event1_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event1_iclr`] module"]
#[doc(alias = "ADC0_INT_EVENT1_ICLR")]
pub type Adc0IntEvent1Iclr = crate::Reg<adc0_int_event1_iclr::Adc0IntEvent1IclrSpec>;
#[doc = "Interrupt clear"]
pub mod adc0_int_event1_iclr;
