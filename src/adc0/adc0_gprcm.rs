#[repr(C)]
#[doc = "ADC0_GPRCM\\[%s\\]"]
#[doc(alias = "ADC0_GPRCM")]
pub struct Adc0Gprcm {
    adc0_pwren: Adc0Pwren,
    adc0_rstctl: Adc0Rstctl,
    adc0_clkcfg: Adc0Clkcfg,
    _reserved3: [u8; 0x08],
    adc0_stat: Adc0Stat,
}
impl Adc0Gprcm {
    #[doc = "0x00 - Power enable"]
    #[inline(always)]
    pub const fn adc0_pwren(&self) -> &Adc0Pwren {
        &self.adc0_pwren
    }
    #[doc = "0x04 - Reset Control"]
    #[inline(always)]
    pub const fn adc0_rstctl(&self) -> &Adc0Rstctl {
        &self.adc0_rstctl
    }
    #[doc = "0x08 - ADC clock configuration Register"]
    #[inline(always)]
    pub const fn adc0_clkcfg(&self) -> &Adc0Clkcfg {
        &self.adc0_clkcfg
    }
    #[doc = "0x14 - Status Register"]
    #[inline(always)]
    pub const fn adc0_stat(&self) -> &Adc0Stat {
        &self.adc0_stat
    }
}
#[doc = "ADC0_PWREN (rw) register accessor: Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_pwren`] module"]
#[doc(alias = "ADC0_PWREN")]
pub type Adc0Pwren = crate::Reg<adc0_pwren::Adc0PwrenSpec>;
#[doc = "Power enable"]
pub mod adc0_pwren;
#[doc = "ADC0_RSTCTL (w) register accessor: Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_rstctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_rstctl`] module"]
#[doc(alias = "ADC0_RSTCTL")]
pub type Adc0Rstctl = crate::Reg<adc0_rstctl::Adc0RstctlSpec>;
#[doc = "Reset Control"]
pub mod adc0_rstctl;
#[doc = "ADC0_CLKCFG (rw) register accessor: ADC clock configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_clkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc0_clkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_clkcfg`] module"]
#[doc(alias = "ADC0_CLKCFG")]
pub type Adc0Clkcfg = crate::Reg<adc0_clkcfg::Adc0ClkcfgSpec>;
#[doc = "ADC clock configuration Register"]
pub mod adc0_clkcfg;
#[doc = "ADC0_STAT (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc0_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0_stat`] module"]
#[doc(alias = "ADC0_STAT")]
pub type Adc0Stat = crate::Reg<adc0_stat::Adc0StatSpec>;
#[doc = "Status Register"]
pub mod adc0_stat;
