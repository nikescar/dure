#[derive(Default)]
pub struct DlgUpdate {
    pub open: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_notes: String,
    pub download_url: String,
    pub do_update: bool,
}
