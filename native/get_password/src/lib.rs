use rpassword::prompt_password;
use std::io::stdin;

mod tty;
use tty::*;

static EMPTY_PASSWORD: &str = "passwords cannot be empty";

#[rustler::nif]
pub fn read_password(prompt: String) -> Result<String, String> {
    let stdio = stdin();
    let mut tty_state = match TtyState::new(&stdio) {
        Ok(state) => state,
        Err(e) => return Err(e),
    };

    tty_state.save();
    tty_state.set_cooked();

    let result = match prompt_password(prompt) {
        Ok(pw) if pw == "" => Err(EMPTY_PASSWORD.to_string()),
        Ok(pw) => Ok(pw),
        Err(err) => Err(err.to_string()),
    };

    tty_state.restore();

    result
}

rustler::init!("Elixir.GetPassword");
