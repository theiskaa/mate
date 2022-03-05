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
