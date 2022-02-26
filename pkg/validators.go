package pkg

import "regexp"

// General regular expressions for validation functions.
var (
	numRegExp = regexp.MustCompile(`\d`)
)

// IsNumber checks if given char is digit number or not.
func IsNumber(ch string) bool {
	return numRegExp.MatchString(ch)
}

// IsPoint checks if given char is "dot" or "comma".
func IsPoint(ch string) bool {
	return ch == "." || ch == ","
}

// IsPlusOrMinus checks if given char is "plus" sign or "minus".
func IsPlusOrMinus(ch string) bool {
	return ch == "+" || ch == "-"
}

// IsProdOrDiv checks if given char is "product" sign or "divide".
func IsProdOrDiv(ch string) bool {
	return ch == "*" || ch == "•" || ch == "/" || ch == ":"
}
