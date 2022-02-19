package lib

import "mate/pkg"

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
	Input        string // Expression Input.
	Char         byte   // Current char under examination.
	position     int    // Current position in input (points to current char).
	readPosition int    // Current reading position in input (after current char).
}

// NewLexer is default way of creating a new Lexer object.
func NewLexer(input string) Lexer {
	lexer := Lexer{Input: input}

	// Activates lexer for further usage.
	// Fills, Char, position, and readPosition.
	lexer.readChar()

	return lexer
}

// Lex loops through the Input, converts each char to a understandable token
// variable, as a result we'd got a list of tokens, which will be used to calculate
// final result of expression or check for validness of expression.
func (l *Lexer) Lex() []Token {
	tokens := []Token{}

	for l.Char != 0 {
		token := l.GenerateToken()
		tokens = append(tokens, token)
	}

	return tokens
}

// GenerateToken converts [l.Char] to token.
func (l *Lexer) GenerateToken() Token {
	l.skipWhitespace()

	// Check if it's supported token or operation sign.
	if lit, isSign := strToTokenType[string(l.Char)]; isSign {
		// TODO: Check next char to determine if it's negative number actually.
		// TODO: Check next chat to determine if it's parentheses sub expression.

		l.readChar()
		return NewToken(lit, lit.toStrValue())
	}

	// Check if it's number
	if pkg.IsNumber(l.Char) {
		num := l.readNumber()
		return NewToken(NUMBER, num)
	}

	ch := l.Char
	l.readChar()

	return NewToken(ILLEGAL, string(ch))
}

// skipWhitespace skips white(empty) spaces and updates state of lexer (by readChar).
func (l *Lexer) skipWhitespace() {
	for l.Char == ' ' || l.Char == '\t' || l.Char == '\n' || l.Char == '\r' {
		l.readChar()
	}
}

// readChar is character reading functionality, which also updates state of lexer.
// If readPosition limit exceeded, appends 0 to lexer's char. (which means end of reading input)
func (l *Lexer) readChar() {
	if l.readPosition >= len(l.Input) {
		l.Char = 0
	} else {
		l.Char = l.Input[l.readPosition]
	}

	// Update positions.
	l.position = l.readPosition
	l.readPosition += 1
}

// readNumber goes and collects from start to end of
// the string number, and returns the full part of that number from input.
//
//  "426.7" actually is a array of [rune]s
//  ┌──────────────────────────────────────┐
//  │ 426.7 ───▶ ['4', '2', '6', '.', '7'] │
//  └──────────────────────────────────────┘
//   To make computer understood that number,
//   We need to find the start and end index
//   from digit to digit.
//
func (l *Lexer) readNumber() string {
	start := l.position

	// Keep reading forward chars if l.Char is number of number-point.
	for pkg.IsNumber(l.Char) || pkg.IsPoint(l.Char) {
		l.readChar()
	}

	return l.Input[start:l.position]
}
