package lib

// Calculator is token-compiler structure model.
// Takes a token array which will be compiled to [float32] answer.
type Calculator struct {
	Input        []Token
	ReadingToken Token
}

// Calculate, is main token-to-number compiler of application.
// Loops through [c.Input] and calculates final answer.
// If there is an error, answer will be "ZERO", and error would be non-nil.
//
//  TODO: Add visual explanation.
//
func (c *Calculator) Calculate() (float32, error) {

	// TODO: Add functionality.

	return 0.42, nil
}

// CalculateSubExp, is a sub token-to-number compiler.
// Which mainly used to calculate "only" sub expression tokens.
// It'll check for sub-expression input from arguments, if it's nil
// function continues calculating a sub-expression from structure -> [l.ReadingToken].
func (c *Calculator) CalculateSubExp() (float32, error) {

	// TODO: Add CalculateSubExp functionality.

	return 0.42, nil
}

// ExecuteOperation, executes operation for X and Y numbers by appropriate operation type.
//
//  Example:
//  ╭───╮        ╭───╮        ╭───────────╮
//  │ X │──▶ 48  │ Y │──▶ 42  │ Operation │──▶ MINUS
//  ╰───╯        ╰───╯        ╰───────────╯
//  ────────────────────────────────────────────────
//                      ╭─────────╮    ╭───╮
//  Answer would be ──▶ │ 48 - 42 │──▶ │ 6 │
//                      ╰─────────╯    ╰───╯
func (c *Calculator) ExecuteOperation(x, y float32, operation TokenType) float32 {
	operations := map[TokenType]float32{
		PLUS:    x + y,
		MINUS:   x - y,
		PRODUCT: x * y,
		DIVIDE:  x / y,
	}

	return operations[operation]
}
