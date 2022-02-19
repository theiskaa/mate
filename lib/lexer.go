package lib

// Lexer is the main lexical converter of the mate.
// It converts given string input(expression) to an array of tokens.
//
// > VISUAL EXAMPLE OF LEXER
//
//      USER INPUT
//     ┌──────────────────────────┐
//     │ (4 * 5 - 5) * 2 + 24 / 2 │
//     └──────────────────────────┘
//
//      OUTPUT OF THE LEXER
//     ┌───────────────────────────────┐
//     │  ┌─────────────────────────┐  │
//     │  │   LPAREN                │─────────▶ First Sub Expression
//     │  │  ┌───────────────────┐  │  │
//     │  │  │   ┌───────────┐   │  │  │
//     │  │  │   │ NUMBER(4) │   │────────────▶ Second Sub Expression
//     │  │  │   │ PRODUCT   │   │  │  │        Which belongs to first sub expression.
//     │  │  │   │ NUMBER(5) │─┐ │  │  │
//     │  │  │   └───────────┘ └────────────┐
//     │  │  │    MINUS          │  │  │    │
//     │  │  │    NUMBER(5)      │  │  │    └─▶ Third Sub Expression
//     │  │  └───────────────────┘  │  │        Which belongs to second sub expression.
//     │  │   RPAREN                │  │
//     │  │   PRODUCT               │  │
//     │  │   NUMBER(2)             │  │
//     │  └─────────────────────────┘  │
//     │   PLUS                        │
//     │  ┌─────────────────────────┐  │    ┌─▶ Fourth Sub Expression
//     │  │  NUMBER(24)             │  │    │
//     │  │  DIVIDE                 │───────┘
//     │  │  NUMBER(2)              │  │
//     │  └─────────────────────────┘  │
//     └───────────────────────────────┘
//
type Lexer struct {
	input        string // Expression input.
	ch           byte   // Current char under examination.
}

// NewLexer is default way of creating a new Lexer object.
func NewLexer(input string) Lexer {
	return Lexer{
		input: input,
	}
}

// Lex loops through the input, converts each char to a understandable token
// variable, as a result we'd got a list of tokens, which will be used to calculate
// final result of expression or check for validness of expression.
func (l *Lexer) Lex() []Token {
	tokens := []Token{}

	for _, ch := range l.input {
		// Skip white(empty) spaces.
		if ch == ' ' || ch == '\n' || ch == '\t' || ch == '\r' {
			continue
		}

		// Update lexer values.
		l.ch = byte(ch)

		token := l.GenerateToken()
		tokens = append(tokens, token)
	}

	return tokens
}

// GenerateToken converts [l.ch] to token.
func (l *Lexer) GenerateToken() Token {
	// Check if it's digit number
	if isNumber(l.ch) {
		// TODO: Read number
	}

	// Check if it's supported token type.
	if lit, isSign := strToTokenType[string(l.ch)]; isSign {
		// TODO: Check next char to determine if it's negative number actually.
		return NewToken(lit, l.ch)
	}

	return NewToken(ILLEGAL, l.ch)
}

// isNumber checks if given char is digit number or not.
func isNumber(ch byte) bool {
	return '0' <= ch && ch <= '9'
}
