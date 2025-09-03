#[repr(C)]
#[doc = "ADC0_INT_EVENT0\\[%s\\]"]
#[doc(alias = "ADC0_INT_EVENT0")]
pub struct Adc0IntEvent0 {
    adc0_int_event0_iidx: Adc0IntEvent0Iidx,
    _reserved1: [u8; 0x04],
    adc0_int_event0_imask: Adc0IntEvent0Imask,
    _reserved2: [u8; 0x04],
    adc0_int_event0_ris: Adc0IntEvent0Ris,
    _reserved3: [u8; 0x04],
    adc0_int_event0_mis: Adc0IntEvent0Mis,
    _reserved4: [u8; 0x04],
    adc0_int_event0_iset: Adc0IntEvent0Iset,
    _reserved5: [u8; 0x04],
    adc0_int_event0_iclr: Adc0IntEvent0Iclr,
}
impl Adc0IntEvent0 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn adc0_int_event0_iidx(&self) -> &Adc0IntEvent0Iidx {
        &self.adc0_int_event0_iidx
    }
    #[doc = "0x08 - Interrupt mask"]
    #[inline(always)]
    pub const fn adc0_int_event0_imask(&self) -> &Adc0IntEvent0Imask {
        &self.adc0_int_event0_imask
    }
    #[doc = "0x10 - Raw interrupt status"]
    #[inline(always)]
    pub const fn adc0_int_event0_ris(&self) -> &Adc0IntEvent0Ris {
        &self.adc0_int_event0_ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn adc0_int_event0_mis(&self) -> &Adc0IntEvent0Mis {
        &self.adc0_int_event0_mis
    }
    #[doc = "0x20 - Interrupt set"]
    #[inline(always)]
    pub const fn adc0_int_event0_iset(&self) -> &Adc0IntEvent0Iset {
        &self.adc0_int_event0_iset
    }
    #[doc = "0x28 - Interrupt clear"]
    #[inline(always)]
    pub const fn adc0_int_event0_iclr(&self) -> &Adc0IntEvent0Iclr {
        &self.adc0_int_event0_iclr
    }
}
#[doc = "ADC0_INT_EVENT0_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event0_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event0_iidx`] module"]
#[doc(alias = "ADC0_INT_EVENT0_IIDX")]
pub type Adc0IntEvent0Iidx = crate::Reg<adc0_int_event0_iidx::Adc0IntEvent0IidxSpec>;
#[doc = "Interrupt index"]
pub mod adc0_int_event0_iidx;
#[doc = "ADC0_INT_EVENT0_IMASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event0_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_int_event0_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event0_imask`] module"]
#[doc(alias = "ADC0_INT_EVENT0_IMASK")]
pub type Adc0IntEvent0Imask = crate::Reg<adc0_int_event0_imask::Adc0IntEvent0ImaskSpec>;
#[doc = "Interrupt mask"]
pub mod adc0_int_event0_imask;
#[doc = "ADC0_INT_EVENT0_RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event0_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event0_ris`] module"]
#[doc(alias = "ADC0_INT_EVENT0_RIS")]
pub type Adc0IntEvent0Ris = crate::Reg<adc0_int_event0_ris::Adc0IntEvent0RisSpec>;
#[doc = "Raw interrupt status"]
pub mod adc0_int_event0_ris;
#[doc = "ADC0_INT_EVENT0_MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event0_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event0_mis`] module"]
#[doc(alias = "ADC0_INT_EVENT0_MIS")]
pub type Adc0IntEvent0Mis = crate::Reg<adc0_int_event0_mis::Adc0IntEvent0MisSpec>;
#[doc = "Masked interrupt status"]
pub mod adc0_int_event0_mis;
#[doc = "ADC0_INT_EVENT0_ISET (w) register accessor: Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_int_event0_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event0_iset`] module"]
#[doc(alias = "ADC0_INT_EVENT0_ISET")]
pub type Adc0IntEvent0Iset = crate::Reg<adc0_int_event0_iset::Adc0IntEvent0IsetSpec>;
#[doc = "Interrupt set"]
pub mod adc0_int_event0_iset;
#[doc = "ADC0_INT_EVENT0_ICLR (w) register accessor: Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_int_event0_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event0_iclr`] module"]
#[doc(alias = "ADC0_INT_EVENT0_ICLR")]
pub type Adc0IntEvent0Iclr = crate::Reg<adc0_int_event0_iclr::Adc0IntEvent0IclrSpec>;
#[doc = "Interrupt clear"]
pub mod adc0_int_event0_iclr;
