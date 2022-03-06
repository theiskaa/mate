package pkg

import (
	"errors"
	"fmt"
)

// IllegalTokenError generates a error message for
// illegal token detecting cases.
func IllegalTokenError(values []interface{}) error {
	err := fmt.Sprintf(
		`
Found an illegal token: (at index %v of parsed tokens)
  • Type: %v
  • Value: %v
  • Sub Tokens: %v
`,
		values...,
	)

	return errors.New(err)
}

// NoOperation generates a error message for no operation sign
// between numbers error.
func NoOperation(values []interface{}) error {
	err := fmt.Sprintf(
		`
There is no operation(+/-/•/:) sign between two tokens.
X: %v, Y: %v
        ╭──────────╮
╭───╮   ▼   ╭───╮  │
│ X │  ...  │ Y │  ╰─▶ Expected an operation token
╰───╯       ╰───╯
`,
		values...,
	)

	return errors.New(err)
}
