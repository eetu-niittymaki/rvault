use crate::cli::PassCommand;
use crate::utils::copy_to_clipboard::copy_to_clipboard;
use crate::utils::password_gen::generate_password;

pub async fn pass(cmd: PassCommand) {
    let password = generate_password();

    if cmd.copy {
        let copy_to_clipboard = copy_to_clipboard(password);
        if copy_to_clipboard {
            println!("Password copied to clipboard")
        } else {
            println!("Error in copying to clipboard!")
        }
    } else {
        println!("{}", password);
    }
}
