package lib

import "mate/pkg"

// TokenType is custom type representation for Token structure.
type TokenType string

// Token is the input's char representation structure.
type Token struct {
	Type      TokenType
	SubTokens []Token
	Literal   string
}

const (
	SUB_EXP = "SUB_EXP"
	NUMBER  = "NUMBER"
	ILLEGAL = "ILLEGAL"

	// Operation tokens.
	PLUS    = "+"
	MINUS   = "-"
	PRODUCT = "*"
	DIVIDE  = "/"

	// Sign tokens.
	LPAREN = "("
	RPAREN = ")"
)

// strToTokenType  is string literal-to-token-constant value map.
var strToTokenType = map[string]TokenType{
	"+": PLUS,
	"-": MINUS,
	"*": PRODUCT,
	"•": PRODUCT,
	"/": DIVIDE,
	":": DIVIDE,
	"(": LPAREN,
	")": RPAREN,
}

// tokenTypeToStr  is token-constant-to-string-literal value map.
var tokenTypeToStr = map[TokenType]string{
	PLUS:    "+",
	MINUS:   "-",
	PRODUCT: "*",
	DIVIDE:  "/",
	LPAREN:  "(",
	RPAREN:  ")",
}


// IsSubExp checks if token is a sub expression token.
func (t *Token) IsSubExp() bool {
	return len(t.SubTokens) == 0
}

// toStrValue is inherited method for TokenType.
// converts a token type variable to string value.
func (t *TokenType) toStrValue() string {
	return tokenTypeToStr[*t]
}

// NewToken is default function which used to create new token variable.
func NewToken(ch string) Token {
	var ty TokenType

	if pkg.IsNumber(ch) {
		ty = NUMBER
	} else {
		if t, ok := strToTokenType[ch]; ok {
			ty = t
		} else {
			ty = ILLEGAL
		}
	}

	return Token{Type: ty, Literal: ch}
}