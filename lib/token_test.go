package lib_test

import (
	"mate/lib"
	"testing"
)

func TestIsSubExp(t *testing.T) {
	tests := []struct {
		token    lib.Token
		expected bool
	}{
		{expected: false, token: lib.NewToken("-2")},
		{expected: false, token: lib.NewToken("*")},
		{expected: false, token: lib.NewToken("-")},
		{expected: false, token: lib.NewToken("+")},
		{expected: false, token: lib.NewToken(":")},
		{
			expected: true,
			token: lib.Token{
				Type: lib.SUB_EXP,
				SubTokens: []lib.Token{
					lib.NewToken("-2"),
					lib.NewToken("*"),
					lib.NewToken("-5"),
				},
			},
		},
	}

	for _, td := range tests {
		got := td.token.IsSubExp()

		if got != td.expected {
			t.Errorf("Sum was different of IsSubExp | Want: %v, Got: %v", td.expected, got)
		}
	}
}

func TestIsIllegal(t *testing.T) {
	tests := []struct {
		token    lib.Token
		expected bool
	}{
		{expected: true, token: lib.NewToken("&")},
		{expected: true, token: lib.NewToken("#")},
		{expected: false, token: lib.NewToken("*")},
		{expected: false, token: lib.NewToken("-")},
		{expected: false, token: lib.NewToken("+")},
		{expected: false, token: lib.NewToken(":")},
	}

	for _, td := range tests {
		got := td.token.IsIllegal()

		if got != td.expected {
			t.Errorf("Sum was different of IsIllegal | Want: %v, Got: %v", td.expected, got)
		}
	}
}

func TestIsNum(t *testing.T) {
	tests := []struct {
		token    lib.Token
		expected bool
	}{
		{expected: true, token: lib.NewToken("6")},
		{expected: true, token: lib.NewToken("42")},
		{expected: true, token: lib.NewToken("7")},
		{expected: false, token: lib.NewToken("*")},
		{expected: false, token: lib.NewToken("-")},
		{expected: false, token: lib.NewToken("+")},
		{expected: false, token: lib.NewToken(":")},
	}

	for _, td := range tests {
		got := td.token.IsNum()

		if got != td.expected {
			t.Errorf("Sum was different of IsNum | Want: %v, Got: %v", td.expected, got)
		}
	}
}

func TestIsPlusOrMinus(t *testing.T) {
	tests := []struct {
		token    lib.Token
		expected bool
	}{
		{expected: true, token: lib.NewToken("-")},
		{expected: true, token: lib.NewToken("+")},
		{expected: false, token: lib.NewToken("-6")},
		{expected: false, token: lib.NewToken("+42")},
		{expected: false, token: lib.NewToken("*")},
		{expected: false, token: lib.NewToken(":")},
	}

	for _, td := range tests {
		got := td.token.IsPlusOrMinus()

		if got != td.expected {
			t.Errorf("Sum was different of IsPlusOrMinus | Want: %v, Got: %v", td.expected, got)
		}
	}
}

func TestIsProdOrDiv(t *testing.T) {
	tests := []struct {
		token    lib.Token
		expected bool
	}{
		{expected: true, token: lib.NewToken("*")},
		{expected: true, token: lib.NewToken("/")},
		{expected: false, token: lib.NewToken("-6")},
		{expected: false, token: lib.NewToken("+42")},
		{expected: false, token: lib.NewToken("-")},
		{expected: false, token: lib.NewToken("+")},
	}

	for _, td := range tests {
		got := td.token.IsProdOrDiv()

		if got != td.expected {
			t.Errorf("Sum was different of IsProdOrDiv | Want: %v, Got: %v", td.expected, got)
		}
	}
}

func TestIsParen(t *testing.T) {
	tests := []struct {
		token    lib.Token
		expected bool
	}{
		{expected: true, token: lib.NewToken(")")},
		{expected: true, token: lib.NewToken("(")},
		{expected: false, token: lib.NewToken("-6")},
		{expected: false, token: lib.NewToken("+42")},
		{expected: false, token: lib.NewToken("-")},
		{expected: false, token: lib.NewToken("+")},
	}

	for _, td := range tests {
		got := td.token.IsParen()

		if got != td.expected {
			t.Errorf("Sum was different of IsParen | Want: %v, Got: %v", td.expected, got)
		}
	}
}

func TestIsLParen(t *testing.T) {
	tests := []struct {
		token    lib.Token
		expected bool
	}{
		{expected: true, token: lib.NewToken("(")},
		{expected: false, token: lib.NewToken(")")},
		{expected: false, token: lib.NewToken("-6")},
		{expected: false, token: lib.NewToken("+42")},
		{expected: false, token: lib.NewToken("-")},
		{expected: false, token: lib.NewToken("+")},
	}

	for _, td := range tests {
		got := td.token.IsLParen()

		if got != td.expected {
			t.Errorf("Sum was different of IsLParen | Want: %v, Got: %v", td.expected, got)
		}
	}
}

func TestIsRParen(t *testing.T) {
	tests := []struct {
		token    lib.Token
		expected bool
	}{
		{expected: true, token: lib.NewToken(")")},
		{expected: false, token: lib.NewToken("(")},
		{expected: false, token: lib.NewToken("-6")},
		{expected: false, token: lib.NewToken("+42")},
		{expected: false, token: lib.NewToken("-")},
		{expected: false, token: lib.NewToken("+")},
	}

	for _, td := range tests {
		got := td.token.IsRParen()

		if got != td.expected {
			t.Errorf("Sum was different of IsRParen | Want: %v, Got: %v", td.expected, got)
		}
	}
}

func TestToStrValue(t *testing.T) {
	tests := []struct {
		token    lib.TokenType
		expected string
	}{
		{expected: "+", token: lib.PLUS},
		{expected: "-", token: lib.MINUS},
		{expected: "*", token: lib.PRODUCT},
		{expected: "/", token: lib.DIVIDE},
		{expected: "(", token: lib.LPAREN},
		{expected: ")", token: lib.RPAREN},
	}

	for _, td := range tests {
		got := td.token.ToStrValue()

		if got != td.expected {
			t.Errorf("Sum was different of ToStrValue | Want: %v, Got: %v", td.expected, got)
		}
	}
}

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
