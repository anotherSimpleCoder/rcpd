use rfd::{MessageDialog, MessageLevel};

pub fn dialog(title: &str, message: &str) -> bool{
    MessageDialog::new()
        .set_title(title)
        .set_description(message)
        .set_buttons(rfd::MessageButtons::YesNo)
        .show()
}

pub fn message(title: &str, message: &str) {
    MessageDialog::new()
        .set_title(title)
        .set_description(message)
        .show();
}

pub fn error(title: &str, message: &str) {
    MessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_title(title)
        .set_description(message)
        .show();
}