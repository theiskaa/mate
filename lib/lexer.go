package lib

// Lexer is the main lexical converter of the mate.
// It converts given string Input(expression) to an array of tokens.
//
// > VISUAL EXAMPLE OF LEXER
//
//      USER INPUT
//     ┌──────────────────────────┐
//     │ (4 * 5 - 5) * 2 + 24 / 2 │
//     └──────────────────────────┘
//
//      OUTPUT OF THE LEXER
//     ┌───────────────────────────────────┐
//     │                                   │    ┌─▶ First Sub Expression
//     │   ┌───────────────────────────┐   │    │
//     │   │                           │────────┘
//     │   │   ┌───────────────────┐   │   │
//     │   │   │                   │─┐ │   │
//     │   │   │   ┌───────────┐   │ │ │   │
//     │   │   │   │ NUMBER(4) │   │ └────────────▶ Second Sub Expression
//     │   │   │   │ PRODUCT   │─┐ │   │   │        Which belongs to first sub expression.
//     │   │   │   │ NUMBER(5) │ │ │   │   │
//     │   │   │   └───────────┘ └──────────────┐
//     │   │   │    MINUS          │   │   │    │
//     │   │   │    NUMBER(5)      │   │   │    └─▶ Third Sub Expression
//     │   │   │                   │   │   │        Which belongs to second sub expression.
//     │   │   └───────────────────┘   │   │
//     │   │                           │   │
//     │   │    PRODUCT                │   │
//     │   │    NUMBER(2)              │   │
//     │   │                           │   │
//     │   └───────────────────────────┘   │
//     │                                   │
//     │    PLUS                           │
//     │                                   │
//     │   ┌──────────────────────────┐    │    ┌─▶ Fourth Sub Expression
//     │   │                          │    │    │
//     │   │  NUMBER(24)              │    │    │
//     │   │  DIVIDE                  │─────────┘
//     │   │  NUMBER(2)               │    │
//     │   │                          │    │
//     │   └──────────────────────────┘    │
//     │                                   │
//     └───────────────────────────────────┘
//
type Lexer struct {
	Input string // Expression Input.
	Char  byte   // Current char under examination.
}

// NewLexer is default way of creating a new Lexer object.
func NewLexer(input string) Lexer {
	return Lexer{Input: input}
}

// Lex loops through the Input, converts each char to a understandable token
// variable, as a result we'd got a list of tokens, which will be used to calculate
// final result of expression or check for validness of expression.
func (l *Lexer) Lex() []Token {
	tokens := []Token{}

	for _, char := range l.Input {
		// Skip white(empty) spaces.
		if char == ' ' || char == '\n' || char == '\t' || char == '\r' {
			continue
		}

		// Update lexer values.
		l.Char = byte(char)

		token := l.GenerateToken()
		tokens = append(tokens, token)
	}

	return tokens
}

// GenerateToken converts [l.Char] to token.
func (l *Lexer) GenerateToken() Token {
	// Check if it's digit number
	if '0' <= l.Char && l.Char <= '9' {
		// TODO: Read number
	}

	// Check if it's supported token type.
	if lit, isSign := strToTokenType[string(l.Char)]; isSign {
		// TODO: Check next char to determine if it's negative number actually.
		return NewToken(lit, lit.toStrValue())
	}

	return NewToken(ILLEGAL, string(l.Char))
}
