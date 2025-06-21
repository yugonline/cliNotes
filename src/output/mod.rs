
use termimad::MadSkin;

pub fn display_success(message: &str) {
    let skin = MadSkin::default();
    skin.print_text(&format!("**✅ Success!** {}", message));
}
