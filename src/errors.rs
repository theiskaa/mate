/// Main structure model for errors of lexer.
pub struct Error<'a> {
    msg: &'a str,
}

impl<'a> Error<'a> {
    pub fn new(msg: &'a str) -> Self {
        Self { msg }
    }

    pub fn to_string(&self) -> String {
        self.msg.to_string()
    }
}
