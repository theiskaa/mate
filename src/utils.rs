///
/// A interface for custom char-type-checking utility methods.
/// Has a various methods (checkers) based on [&str].
///
pub trait ChUtils<'a> {
    /// Checks if the given [&self] object is number or not.
    fn is_number(&'a self) -> bool;

    /// Checks if the given [&self] object is point(comma, dot) or not.
    ///
    /// Like <.> in 3.14 or <,> in 3,14
    fn is_point(&'a self) -> bool;

    /// Checks if the given [&self] object is plus sign or minus sign.
    ///
    /// Plus signs   --> <+>
    /// Minus signs  --> <->
    fn is_plus_or_minus(&'a self) -> bool;

    /// Checks if the given [&self] object is division sign or multiplication sign.
    ///
    /// Division signs        --> <:> and </>
    /// Multiplication signs  --> <*> and <•>
    fn is_div_or_prod(&'a self) -> bool;

    /// A function that combines [is_plus_or_minus] and [is_div_or_prod].
    /// So, it checks if [&self] object is operation sign or not.
    ///
    /// Plus signs            --> <+>
    /// Minus signs           --> <->
    /// Division signs        --> <:> and </>
    /// Multiplication signs  --> <*> and <•>
    fn is_operation_sign(&'a self) -> bool;
}

impl<'a> ChUtils<'a> for &'a str {
    // TODO: Check with regular expression matchers.
    fn is_number(&self) -> bool {
        for c in self.trim().chars() {
            if c.is_numeric() {
                continue;
            }

            return false;
        }

        true
    }

    fn is_point(&self) -> bool {
        self.trim().eq(".") || self.trim().eq(",")
    }

    fn is_plus_or_minus(&self) -> bool {
        self.trim().eq("+") || self.trim().eq("-")
    }

    fn is_div_or_prod(&self) -> bool {
        let is_div: bool = self.trim().eq(":") || self.trim().eq("/");
        let is_prod: bool = self.trim().eq("*") || self.trim().eq("•");

        is_div || is_prod
    }

    fn is_operation_sign(&self) -> bool {
        self.is_plus_or_minus() || self.is_div_or_prod()
    }
}
