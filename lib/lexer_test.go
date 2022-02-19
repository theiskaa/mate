package lib_test

import (
	"mate/lib"
	"testing"
)

// compareTokens is a simple recursive function used to check two token lists equality.
func compareTokens(a, b []lib.Token, t *testing.T) {
	if len(a) != len(b) {
		t.Errorf("The length of Sum of Lex was different, Want: %v, Got: %v", len(a), len(b))
		return
	}

	for i := 0; i < len(a); i++ {
		got := a[i]
		expected := b[i]

		if got.Type != expected.Type || got.Literal != expected.Literal {
			t.Errorf("Sum of Lex was different at index %v, Want: %v, Got: %v", i, expected, got)
		}

		compareTokens(got.SubTokens, expected.SubTokens, t)
	}
}

func TestLex(t *testing.T) {
	tests := []struct {
		testname string
		input    string
		expected []lib.Token
	}{
		{
			testname: "should parse one-root base expression",
			input:    "2 + 4.5 - 5",
			expected: []lib.Token{
				{Type: lib.NUMBER, Literal: "2"},
				{Type: lib.PLUS, Literal: "+"},
				{Type: lib.NUMBER, Literal: "4.5"},
				{Type: lib.MINUS, Literal: "-"},
				{Type: lib.NUMBER, Literal: "5"},
			},
		},
		{
			testname: "should parse multi-root base expression",
			input:    "(4 * 5 - 5) * 2 + 24 / 2",
			expected: []lib.Token{
				{Type: lib.SUB_EXP, SubTokens: []lib.Token{
					{Type: lib.SUB_EXP, SubTokens: []lib.Token{
						{Type: lib.SUB_EXP, SubTokens: []lib.Token{
							{Type: lib.NUMBER, Literal: "4"},
							{Type: lib.PRODUCT, Literal: "*"},
							{Type: lib.NUMBER, Literal: "5"},
						}},
						{Type: lib.MINUS, Literal: "-"},
						{Type: lib.NUMBER, Literal: "5"},
					}},
					{Type: lib.PRODUCT, Literal: "*"},
					{Type: lib.NUMBER, Literal: "2"},
				}},
				{Type: lib.PLUS, Literal: "+"},
				{Type: lib.SUB_EXP, SubTokens: []lib.Token{
					{Type: lib.NUMBER, Literal: "24"},
					{Type: lib.DIVIDE, Literal: "/"},
					{Type: lib.NUMBER, Literal: "2"},
				}},
			},
		},
		{
			testname: "should parse multi-root base & mixed-sign expression",
			input:    "(24 / 4) + 2 - (-2 * -5)",
			expected: []lib.Token{
				{Type: lib.SUB_EXP, SubTokens: []lib.Token{
					{Type: lib.NUMBER, Literal: "24"},
					{Type: lib.DIVIDE, Literal: "/"},
					{Type: lib.NUMBER, Literal: "4"},
				}},
				{Type: lib.PLUS, Literal: "+"},
				{Type: lib.NUMBER, Literal: "2"},
				{Type: lib.MINUS, Literal: "-"},
				{Type: lib.SUB_EXP, SubTokens: []lib.Token{
					{Type: lib.NUMBER, Literal: "-2"},
					{Type: lib.PRODUCT, Literal: "*"},
					{Type: lib.NUMBER, Literal: "-5"},
				}},
			},
		},
	}

	for _, td := range tests {
		lexer := lib.NewLexer(td.input)

		got := lexer.Lex()
		compareTokens(got, td.expected, t)
	}
}

func TestGenerateToken(t *testing.T) {
	tests := []struct {
		input    rune
		expected lib.Token
	}{
		{
			input:    '+',
			expected: lib.Token{Type: lib.PLUS, Literal: "+"},
		},
		{
			input:    '-',
			expected: lib.Token{Type: lib.MINUS, Literal: "-"},
		},
		{
			input:    ':',
			expected: lib.Token{Type: lib.DIVIDE, Literal: "/"},
		},
		{
			input:    '/',
			expected: lib.Token{Type: lib.DIVIDE, Literal: "/"},
		},
		{
			input:    '*',
			expected: lib.Token{Type: lib.PRODUCT, Literal: "*"},
		},
	}

	for _, td := range tests {
		lexer := lib.NewLexer(string(td.input))

		// Set input to the lexer's char.
		lexer.Char = byte(td.input)

		got := lexer.GenerateToken()
		if got.Type != td.expected.Type || got.Literal != td.expected.Literal {
			t.Errorf("Sum of GenerateToken was different, Want: %v, Got: %v", td.expected, got)
		}
	}
}
