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
	input string // Expression input.
	ch    string // Current char under examination.
}

// NewLexer is default way of creating a new Lexer object.
func NewLexer(input string) Lexer {
	return Lexer{
		input: input,
	}
}

// Parse loops through the input, converts each char to a understandable token
// variable, as a result we'd got a list of tokens, which will be used to calculate
// final result of expression or check for validness of expression.
func (l *Lexer) Parse() []Token {
	tokens := []Token{}

	for _, ch := range l.input {
		// Never mind empty/white spaces.
		if ch == ' ' || ch == '\n' || ch == '\t' || ch == '\r' {
			continue
		}

		// TODO: Implement NextToken functionality.
	}

	return tokens
}

// NextToken checks [l.ch] element and converts it token
func (l *Lexer) NextToken() Token {
	// TODO: Add functionality.
	return Token{}
}
