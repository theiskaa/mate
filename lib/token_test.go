package lib_test

import (
	"mate/lib"
	"testing"
)

func TestNewToken(t *testing.T) {
	type args struct {
		tokenType lib.TokenType
		ch        string
	}

	tests := []struct {
		a        args
		expected lib.Token
	}{
		{
			a: args{
				tokenType: lib.NUMBER,
				ch:        "5",
			},
			expected: lib.Token{
				Type:    lib.NUMBER,
				Literal: "5",
			},
		},
		{
			a: args{
				tokenType: lib.ILLEGAL,
				ch:        "$",
			},
			expected: lib.Token{
				Type:    lib.ILLEGAL,
				Literal: "$",
			},
		},
	}

	for _, td := range tests {
		got := lib.NewToken(td.a.tokenType, td.a.ch)

		if got.Type != td.expected.Type || got.Literal != td.expected.Literal {
			t.Errorf("Sum of NewToken was different, Want: %v, Got: %v", td.expected, got)
		}
	}
}
