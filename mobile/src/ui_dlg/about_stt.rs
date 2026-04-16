#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Default)]
pub struct DlgAbout {
    pub open: bool,
    pub do_check_update: bool,
    pub do_perform_update: bool,
}
