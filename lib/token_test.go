package lib_test

import (
	"mate/lib"
	"testing"
)

func TestNewToken(t *testing.T) {
	tests := []struct {
		ch       string
		expected lib.Token
	}{
		{
			ch: "+",
			expected: lib.Token{
				Type:    lib.PLUS,
				Literal: "+",
			},
		},
		{
			ch: "*",
			expected: lib.Token{
				Type:    lib.PRODUCT,
				Literal: "*",
			},
		},
		{
			ch: "-5",
			expected: lib.Token{
				Type:    lib.NUMBER,
				Literal: "-5",
			},
		},
		{
			ch: "$",
			expected: lib.Token{
				Type:    lib.ILLEGAL,
				Literal: "$",
			},
		},
	}

	for _, td := range tests {
		got := lib.NewToken(td.ch)

		if got.Type != td.expected.Type || got.Literal != td.expected.Literal {
			t.Errorf("Sum of NewToken was different, Want: %v, Got: %v", td.expected, got)
		}
	}
}
