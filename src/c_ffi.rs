use std::ffi::c_char;

pub const SAVES_PATH: &str = "data/saves/";
pub const VOCAB_PATH: &str = "CowsAndBulls-in-C/data/words/5_letters_en_words.txt";

#[link(name = "cab_api", kind = "static")]
unsafe extern "C" {
    pub unsafe fn start_new_game();
    pub unsafe fn setup_game();
    pub unsafe fn play_turn_charptr(input_string: *mut c_char) -> *mut c_char;
    pub unsafe fn set_saves_folder_path(saves_path: *const c_char) -> bool;
    pub unsafe fn set_vocabolary_file_path(vocab_path: *const c_char) -> bool;
}

pub fn play_turn_wrapper(s: &str) -> String {
    //&str --> CString (stringa C di Rust)
    let input_string = std::ffi::CString::new(s).unwrap();

    //CString --> *mut CString
    let input_ptr = input_string.as_ptr() as *mut _;
    
    let result_ptr = unsafe { play_turn_charptr(input_ptr)};
    
    //*mut c_char --> &CStr
    let result_string = unsafe {std::ffi::CStr::from_ptr(result_ptr) };

    // &CStr --> String
    result_string
        .to_string_lossy()
        .into_owned()
}

pub fn set_paths() -> bool {
    unsafe {
        let saves = std::ffi::CString::new(SAVES_PATH).unwrap();
        let vocab = std::ffi::CString::new(VOCAB_PATH).unwrap();
        
        set_saves_folder_path(saves.as_ptr())
        && set_vocabolary_file_path(vocab.as_ptr())
    }
}