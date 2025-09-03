#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    adc0_fsub_0: Adc0Fsub0,
    _reserved1: [u8; 0x40],
    adc0_fpub_1: Adc0Fpub1,
    _reserved2: [u8; 0x03b8],
    adc0_gprcm: [Adc0Gprcm; 1],
    _reserved3: [u8; 0x0808],
    adc0_int_event0: [Adc0IntEvent0; 1],
    _reserved4: [u8; 0x04],
    adc0_int_event1: [Adc0IntEvent1; 1],
    _reserved5: [u8; 0x04],
    adc0_int_event2: [Adc0IntEvent2; 1],
    _reserved6: [u8; 0x34],
    adc0_evt_mode: Adc0EvtMode,
    _reserved7: [u8; 0x18],
    adc0_desc: Adc0Desc,
    adc0_ctl0: Adc0Ctl0,
    adc0_ctl1: Adc0Ctl1,
    adc0_ctl2: Adc0Ctl2,
    _reserved11: [u8; 0x04],
    adc0_clkfreq: Adc0Clkfreq,
    adc0_scomp0: Adc0Scomp0,
    adc0_scomp1: Adc0Scomp1,
    _reserved14: [u8; 0x2c],
    adc0_wclow: Adc0Wclow,
    _reserved15: [u8; 0x04],
    adc0_wchigh: Adc0Wchigh,
    _reserved16: [u8; 0x2c],
    adc0_memctl: [Adc0Memctl; 12],
    _reserved17: [u8; 0x0190],
    adc0_status: Adc0Status,
}
impl RegisterBlock {
    #[doc = "0x400 - Subscriber Configuration Register."]
    #[inline(always)]
    pub const fn adc0_fsub_0(&self) -> &Adc0Fsub0 {
        &self.adc0_fsub_0
    }
    #[doc = "0x444 - Publisher Configuration Register."]
    #[inline(always)]
    pub const fn adc0_fpub_1(&self) -> &Adc0Fpub1 {
        &self.adc0_fpub_1
    }
    #[doc = "0x800..0x818 - ADC0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub const fn adc0_gprcm(&self, n: usize) -> &Adc0Gprcm {
        &self.adc0_gprcm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - ADC0_GPRCM\\[%s\\]"]
    #[inline(always)]
    pub fn adc0_gprcm_iter(&self) -> impl Iterator<Item = &Adc0Gprcm> {
        self.adc0_gprcm.iter()
    }
    #[doc = "0x1020..0x104c - ADC0_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub const fn adc0_int_event0(&self, n: usize) -> &Adc0IntEvent0 {
        &self.adc0_int_event0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1020..0x104c - ADC0_INT_EVENT0\\[%s\\]"]
    #[inline(always)]
    pub fn adc0_int_event0_iter(&self) -> impl Iterator<Item = &Adc0IntEvent0> {
        self.adc0_int_event0.iter()
    }
    #[doc = "0x1050..0x107c - ADC0_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub const fn adc0_int_event1(&self, n: usize) -> &Adc0IntEvent1 {
        &self.adc0_int_event1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1050..0x107c - ADC0_INT_EVENT1\\[%s\\]"]
    #[inline(always)]
    pub fn adc0_int_event1_iter(&self) -> impl Iterator<Item = &Adc0IntEvent1> {
        self.adc0_int_event1.iter()
    }
    #[doc = "0x1080..0x10ac - ADC0_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub const fn adc0_int_event2(&self, n: usize) -> &Adc0IntEvent2 {
        &self.adc0_int_event2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1080..0x10ac - ADC0_INT_EVENT2\\[%s\\]"]
    #[inline(always)]
    pub fn adc0_int_event2_iter(&self) -> impl Iterator<Item = &Adc0IntEvent2> {
        self.adc0_int_event2.iter()
    }
    #[doc = "0x10e0 - Event Mode"]
    #[inline(always)]
    pub const fn adc0_evt_mode(&self) -> &Adc0EvtMode {
        &self.adc0_evt_mode
    }
    #[doc = "0x10fc - Module Description"]
    #[inline(always)]
    pub const fn adc0_desc(&self) -> &Adc0Desc {
        &self.adc0_desc
    }
    #[doc = "0x1100 - Control Register 0"]
    #[inline(always)]
    pub const fn adc0_ctl0(&self) -> &Adc0Ctl0 {
        &self.adc0_ctl0
    }
    #[doc = "0x1104 - Control Register 1"]
    #[inline(always)]
    pub const fn adc0_ctl1(&self) -> &Adc0Ctl1 {
        &self.adc0_ctl1
    }
    #[doc = "0x1108 - Control Register 2"]
    #[inline(always)]
    pub const fn adc0_ctl2(&self) -> &Adc0Ctl2 {
        &self.adc0_ctl2
    }
    #[doc = "0x1110 - Sample Clock Frequency Range Register"]
    #[inline(always)]
    pub const fn adc0_clkfreq(&self) -> &Adc0Clkfreq {
        &self.adc0_clkfreq
    }
    #[doc = "0x1114 - Sample Time Compare 0 Register"]
    #[inline(always)]
    pub const fn adc0_scomp0(&self) -> &Adc0Scomp0 {
        &self.adc0_scomp0
    }
    #[doc = "0x1118 - Sample Time Compare 1 Register"]
    #[inline(always)]
    pub const fn adc0_scomp1(&self) -> &Adc0Scomp1 {
        &self.adc0_scomp1
    }
    #[doc = "0x1148 - Window Comparator Low Threshold Register"]
    #[inline(always)]
    pub const fn adc0_wclow(&self) -> &Adc0Wclow {
        &self.adc0_wclow
    }
    #[doc = "0x1150 - Window Comparator High Threshold Register"]
    #[inline(always)]
    pub const fn adc0_wchigh(&self) -> &Adc0Wchigh {
        &self.adc0_wchigh
    }
    #[doc = "0x1180..0x11b0 - Conversion Memory Control Register"]
    #[inline(always)]
    pub const fn adc0_memctl(&self, n: usize) -> &Adc0Memctl {
        &self.adc0_memctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1180..0x11b0 - Conversion Memory Control Register"]
    #[inline(always)]
    pub fn adc0_memctl_iter(&self) -> impl Iterator<Item = &Adc0Memctl> {
        self.adc0_memctl.iter()
    }
    #[doc = "0x1340 - Status Register"]
    #[inline(always)]
    pub const fn adc0_status(&self) -> &Adc0Status {
        &self.adc0_status
    }
}
#[doc = "ADC0_FSUB_0 (rw) register accessor: Subscriber Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_fsub_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_fsub_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_fsub_0`] module"]
#[doc(alias = "ADC0_FSUB_0")]
pub type Adc0Fsub0 = crate::Reg<adc0_fsub_0::Adc0Fsub0Spec>;
#[doc = "Subscriber Configuration Register."]
pub mod adc0_fsub_0;
#[doc = "ADC0_FPUB_1 (rw) register accessor: Publisher Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_fpub_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_fpub_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_fpub_1`] module"]
#[doc(alias = "ADC0_FPUB_1")]
pub type Adc0Fpub1 = crate::Reg<adc0_fpub_1::Adc0Fpub1Spec>;
#[doc = "Publisher Configuration Register."]
pub mod adc0_fpub_1;
#[doc = "ADC0_GPRCM\\[%s\\]"]
pub use self::adc0_gprcm::Adc0Gprcm;
#[doc = r"Cluster"]
#[doc = "ADC0_GPRCM\\[%s\\]"]
pub mod adc0_gprcm;
#[doc = "ADC0_INT_EVENT0\\[%s\\]"]
pub use self::adc0_int_event0::Adc0IntEvent0;
#[doc = r"Cluster"]
#[doc = "ADC0_INT_EVENT0\\[%s\\]"]
pub mod adc0_int_event0;
#[doc = "ADC0_INT_EVENT1\\[%s\\]"]
pub use self::adc0_int_event1::Adc0IntEvent1;
#[doc = r"Cluster"]
#[doc = "ADC0_INT_EVENT1\\[%s\\]"]
pub mod adc0_int_event1;
#[doc = "ADC0_INT_EVENT2\\[%s\\]"]
pub use self::adc0_int_event2::Adc0IntEvent2;
#[doc = r"Cluster"]
#[doc = "ADC0_INT_EVENT2\\[%s\\]"]
pub mod adc0_int_event2;
#[doc = "ADC0_EVT_MODE (r) register accessor: Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_evt_mode::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_evt_mode`] module"]
#[doc(alias = "ADC0_EVT_MODE")]
pub type Adc0EvtMode = crate::Reg<adc0_evt_mode::Adc0EvtModeSpec>;
#[doc = "Event Mode"]
pub mod adc0_evt_mode;
#[doc = "ADC0_DESC (r) register accessor: Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_desc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_desc`] module"]
#[doc(alias = "ADC0_DESC")]
pub type Adc0Desc = crate::Reg<adc0_desc::Adc0DescSpec>;
#[doc = "Module Description"]
pub mod adc0_desc;
#[doc = "ADC0_CTL0 (rw) register accessor: Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_ctl0`] module"]
#[doc(alias = "ADC0_CTL0")]
pub type Adc0Ctl0 = crate::Reg<adc0_ctl0::Adc0Ctl0Spec>;
#[doc = "Control Register 0"]
pub mod adc0_ctl0;
#[doc = "ADC0_CTL1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_ctl1`] module"]
#[doc(alias = "ADC0_CTL1")]
pub type Adc0Ctl1 = crate::Reg<adc0_ctl1::Adc0Ctl1Spec>;
#[doc = "Control Register 1"]
pub mod adc0_ctl1;
#[doc = "ADC0_CTL2 (rw) register accessor: Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_ctl2`] module"]
#[doc(alias = "ADC0_CTL2")]
pub type Adc0Ctl2 = crate::Reg<adc0_ctl2::Adc0Ctl2Spec>;
#[doc = "Control Register 2"]
pub mod adc0_ctl2;
#[doc = "ADC0_CLKFREQ (rw) register accessor: Sample Clock Frequency Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_clkfreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_clkfreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_clkfreq`] module"]
#[doc(alias = "ADC0_CLKFREQ")]
pub type Adc0Clkfreq = crate::Reg<adc0_clkfreq::Adc0ClkfreqSpec>;
#[doc = "Sample Clock Frequency Range Register"]
pub mod adc0_clkfreq;
#[doc = "ADC0_SCOMP0 (rw) register accessor: Sample Time Compare 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_scomp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_scomp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_scomp0`] module"]
#[doc(alias = "ADC0_SCOMP0")]
pub type Adc0Scomp0 = crate::Reg<adc0_scomp0::Adc0Scomp0Spec>;
#[doc = "Sample Time Compare 0 Register"]
pub mod adc0_scomp0;
#[doc = "ADC0_SCOMP1 (rw) register accessor: Sample Time Compare 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_scomp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_scomp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_scomp1`] module"]
#[doc(alias = "ADC0_SCOMP1")]
pub type Adc0Scomp1 = crate::Reg<adc0_scomp1::Adc0Scomp1Spec>;
#[doc = "Sample Time Compare 1 Register"]
pub mod adc0_scomp1;
#[doc = "ADC0_WCLOW (rw) register accessor: Window Comparator Low Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_wclow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_wclow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_wclow`] module"]
#[doc(alias = "ADC0_WCLOW")]
pub type Adc0Wclow = crate::Reg<adc0_wclow::Adc0WclowSpec>;
#[doc = "Window Comparator Low Threshold Register"]
pub mod adc0_wclow;
#[doc = "ADC0_WCHIGH (rw) register accessor: Window Comparator High Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_wchigh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_wchigh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_wchigh`] module"]
#[doc(alias = "ADC0_WCHIGH")]
pub type Adc0Wchigh = crate::Reg<adc0_wchigh::Adc0WchighSpec>;
#[doc = "Window Comparator High Threshold Register"]
pub mod adc0_wchigh;
#[doc = "ADC0_MEMCTL (rw) register accessor: Conversion Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_memctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_memctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_memctl`] module"]
#[doc(alias = "ADC0_MEMCTL")]
pub type Adc0Memctl = crate::Reg<adc0_memctl::Adc0MemctlSpec>;
#[doc = "Conversion Memory Control Register"]
pub mod adc0_memctl;
#[doc = "ADC0_STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_status`] module"]
#[doc(alias = "ADC0_STATUS")]
pub type Adc0Status = crate::Reg<adc0_status::Adc0StatusSpec>;
#[doc = "Status Register"]
pub mod adc0_status;
