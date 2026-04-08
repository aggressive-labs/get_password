use rustix::fd::AsFd;
use rustix::termios::OptionalActions::Drain;
use rustix::termios::{isatty, tcgetattr, tcsetattr};
use rustix::termios::{InputModes, LocalModes, OutputModes, Termios};

pub struct TtyState<'a, Fd> {
    fd: &'a Fd,
    saved_mode: Option<Termios>,
    cooked_mode: Option<Termios>,
}

impl<'a, Fd: AsFd> TtyState<'a, Fd> {
    const MUST_SAVE: &'static str = "TTY state wasn't saved before setting or restoring mode";

    pub fn new(stdio: &'a Fd) -> Result<Self, String> {
        if !isatty(stdio) {
            return Err("not a tty".to_string());
        }
        Ok(Self {
            fd: stdio,
            saved_mode: None,
            cooked_mode: None,
        })
    }

    pub fn save(&mut self) {
        let save = tcgetattr(self.fd).unwrap();

        let mut cook = save.clone();
        cook.input_modes
            .insert(InputModes::ICRNL | InputModes::IXON);
        cook.output_modes.insert(OutputModes::OPOST);
        cook.local_modes
            .insert(LocalModes::ECHO | LocalModes::ICANON);

        self.saved_mode = Some(save);
        self.cooked_mode = Some(cook);
    }

    pub fn set_cooked(&mut self) {
        tcsetattr(
            self.fd,
            Drain,
            self.cooked_mode.as_ref().expect(Self::MUST_SAVE),
        )
        .unwrap()
    }

    pub fn restore(self) {
        tcsetattr(
            self.fd,
            Drain,
            self.saved_mode.as_ref().expect(Self::MUST_SAVE),
        )
        .unwrap()
    }
}
