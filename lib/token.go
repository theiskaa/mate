package lib

// TokenType is custom type representation for Token structure.
type TokenType string

// Token is the input's char representation structure.
type Token struct {
	Type    TokenType
	Literal string
}

// NewToken is default function which used to create new token variable.
func NewToken(tokenType TokenType, ch byte) Token {
	return Token{Type: tokenType, Literal: string(ch)}
}

const (
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
