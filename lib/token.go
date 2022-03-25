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
	return len(t.SubTokens) > 0
}

// IsIllegal checks if token is a sub expression token.
func (t *Token) IsIllegal() bool {
	return t.Type == ILLEGAL
}

// IsNum checks if token is a number token or not.
func (t *Token) IsNum() bool {
	return t.Type == NUMBER
}

// IsOperationSign checks if token is a Operation{+, -, etc} token or not.
func (t *Token) IsOperationSign() bool {
	return t.IsPlusOrMinus() || t.IsProdOrDiv()
}

// IsPlusOrMinus checks if token is a PLUS token or MINUS token.
func (t *Token) IsPlusOrMinus() bool {
	return t.Type == PLUS || t.Type == MINUS
}

// IsProdOrDiv checks if token is a PRODUCT token or DIVIDE token.
func (t *Token) IsProdOrDiv() bool {
	return t.Type == PRODUCT || t.Type == DIVIDE
}

func (t *Token) IsParen() bool {
	return t.IsLParen() || t.IsRParen()
}

// IsLParen checks if token is a left parentheses token.
func (t *Token) IsLParen() bool {
	return t.Type == LPAREN
}

// IsRParen checks if token is a right parentheses token.
func (t *Token) IsRParen() bool {
	return t.Type == RPAREN
}

// toStrValue is inherited method for TokenType.
// converts a token type variable to string value.
func (t *TokenType) ToStrValue() string {
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

// NewSubToken is default function which used to create new sub token variable.
func NewSubToken(data []Token) Token {
	return Token{Type: SUB_EXP, SubTokens: data}
}
