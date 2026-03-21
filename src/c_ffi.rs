#[link(name = "cab_api", kind = "static")]
unsafe extern "C" {
    pub unsafe fn start_new_game();
    pub unsafe fn setup_game();
}