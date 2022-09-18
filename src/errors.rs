//
// Copyright 2022-present theiskaa. All rights reserved.
// Use of this source code is governed by MIT license
// that can be found in the LICENSE file.
//

// Main structure model for errors of lexer.
#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let result: Error = Error::new("test message");
        assert_eq!(result.msg, "test message");
    }

    #[test]
    fn to_string() {
        let error: Error = Error::new("A new message");
        assert_eq!(error.to_string(), error.msg)
    }
}
