package lib

// Calculate takes string input(expression).
// Uses lexer structure to parse input, and calculator structure
// to calculate tokens.
// If there is an error, result will be "ZERO".
//
//   User Input                  Output of the Lexer
//  ╭─────────────────────╮     ╭───────╮ ╭───╮ ╭───╮ ╭───╮ ╭─────────╮
//  │ 6 * 7 - 2 + 0.5 * 4 │ ──▶ │ 6 * 7 │ │ - │ │ 2 │ │ + │ │ 0.5 * 4 │──╮
//  ╰─────────────────────╯     ╰───────╯ ╰───╯ ╰───╯ ╰───╯ ╰─────────╯  │
//  ╭────────────────────────────────────────────────────────────────────╯
//  │ Output of the Calculator
//  ╰───────────────────────────▶ 42
//
func Calculate(input string) (float32, error) {
	// Re-generate lexer with input.
	lexer := NewLexer(input)

	// Parse input.
	tokens := lexer.Lex()

	// Re-generate calculator with tokens.
	calculator := NewCalculator(tokens)

	return calculator.Calculate(tokens)
}
