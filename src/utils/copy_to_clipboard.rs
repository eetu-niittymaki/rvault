use cli_clipboard;
use std::panic;

pub fn copy_to_clipboard(string: String) -> bool {
    cli_clipboard::set_contents(string.to_owned()).unwrap();

    let result = panic::catch_unwind(|| {
        assert_eq!(cli_clipboard::get_contents().unwrap(), string);
    });
    
    match result {
        Ok(_) => return true,
        Err(_) =>  return false
    }
}
