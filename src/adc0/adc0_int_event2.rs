#[repr(C)]
#[doc = "ADC0_INT_EVENT2\\[%s\\]"]
#[doc(alias = "ADC0_INT_EVENT2")]
pub struct Adc0IntEvent2 {
    adc0_int_event2_iidx: Adc0IntEvent2Iidx,
    _reserved1: [u8; 0x04],
    adc0_int_event2_imask: Adc0IntEvent2Imask,
    _reserved2: [u8; 0x04],
    adc0_int_event2_ris: Adc0IntEvent2Ris,
    _reserved3: [u8; 0x04],
    adc0_int_event2_mis: Adc0IntEvent2Mis,
    _reserved4: [u8; 0x04],
    adc0_int_event2_iset: Adc0IntEvent2Iset,
    _reserved5: [u8; 0x04],
    adc0_int_event2_iclr: Adc0IntEvent2Iclr,
}
impl Adc0IntEvent2 {
    #[doc = "0x00 - Interrupt index"]
    #[inline(always)]
    pub const fn adc0_int_event2_iidx(&self) -> &Adc0IntEvent2Iidx {
        &self.adc0_int_event2_iidx
    }
    #[doc = "0x08 - Interrupt mask extension"]
    #[inline(always)]
    pub const fn adc0_int_event2_imask(&self) -> &Adc0IntEvent2Imask {
        &self.adc0_int_event2_imask
    }
    #[doc = "0x10 - Raw interrupt status extension"]
    #[inline(always)]
    pub const fn adc0_int_event2_ris(&self) -> &Adc0IntEvent2Ris {
        &self.adc0_int_event2_ris
    }
    #[doc = "0x18 - Masked interrupt status extension"]
    #[inline(always)]
    pub const fn adc0_int_event2_mis(&self) -> &Adc0IntEvent2Mis {
        &self.adc0_int_event2_mis
    }
    #[doc = "0x20 - Interrupt set extension"]
    #[inline(always)]
    pub const fn adc0_int_event2_iset(&self) -> &Adc0IntEvent2Iset {
        &self.adc0_int_event2_iset
    }
    #[doc = "0x28 - Interrupt clear extension"]
    #[inline(always)]
    pub const fn adc0_int_event2_iclr(&self) -> &Adc0IntEvent2Iclr {
        &self.adc0_int_event2_iclr
    }
}
#[doc = "ADC0_INT_EVENT2_IIDX (r) register accessor: Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event2_iidx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event2_iidx`] module"]
#[doc(alias = "ADC0_INT_EVENT2_IIDX")]
pub type Adc0IntEvent2Iidx = crate::Reg<adc0_int_event2_iidx::Adc0IntEvent2IidxSpec>;
#[doc = "Interrupt index"]
pub mod adc0_int_event2_iidx;
#[doc = "ADC0_INT_EVENT2_IMASK (rw) register accessor: Interrupt mask extension\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event2_imask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_int_event2_imask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event2_imask`] module"]
#[doc(alias = "ADC0_INT_EVENT2_IMASK")]
pub type Adc0IntEvent2Imask = crate::Reg<adc0_int_event2_imask::Adc0IntEvent2ImaskSpec>;
#[doc = "Interrupt mask extension"]
pub mod adc0_int_event2_imask;
#[doc = "ADC0_INT_EVENT2_RIS (r) register accessor: Raw interrupt status extension\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event2_ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event2_ris`] module"]
#[doc(alias = "ADC0_INT_EVENT2_RIS")]
pub type Adc0IntEvent2Ris = crate::Reg<adc0_int_event2_ris::Adc0IntEvent2RisSpec>;
#[doc = "Raw interrupt status extension"]
pub mod adc0_int_event2_ris;
#[doc = "ADC0_INT_EVENT2_MIS (r) register accessor: Masked interrupt status extension\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_int_event2_mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event2_mis`] module"]
#[doc(alias = "ADC0_INT_EVENT2_MIS")]
pub type Adc0IntEvent2Mis = crate::Reg<adc0_int_event2_mis::Adc0IntEvent2MisSpec>;
#[doc = "Masked interrupt status extension"]
pub mod adc0_int_event2_mis;
#[doc = "ADC0_INT_EVENT2_ISET (w) register accessor: Interrupt set extension\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_int_event2_iset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event2_iset`] module"]
#[doc(alias = "ADC0_INT_EVENT2_ISET")]
pub type Adc0IntEvent2Iset = crate::Reg<adc0_int_event2_iset::Adc0IntEvent2IsetSpec>;
#[doc = "Interrupt set extension"]
pub mod adc0_int_event2_iset;
#[doc = "ADC0_INT_EVENT2_ICLR (w) register accessor: Interrupt clear extension\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_int_event2_iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_int_event2_iclr`] module"]
#[doc(alias = "ADC0_INT_EVENT2_ICLR")]
pub type Adc0IntEvent2Iclr = crate::Reg<adc0_int_event2_iclr::Adc0IntEvent2IclrSpec>;
#[doc = "Interrupt clear extension"]
pub mod adc0_int_event2_iclr;
