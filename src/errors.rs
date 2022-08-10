/// Main structure model for errors of lexer.
pub struct Error<'a> {
    msg: &'a str,
}

impl<'a> Error<'a> {
    /// A alias for creating [Error] models.
    pub fn new(msg: &'a str) -> Self {
        return Self { msg };
    }
}
