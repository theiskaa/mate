package pkg

// IsNumber checks if given char is digit number or not.
func IsNumber(ch byte) bool {
	return '0' <= ch && ch <= '9'
}

// IsPoint checks if given char is dot or comma.
func IsPoint(ch byte) bool {
	return ch == '.' || ch == ','
}
