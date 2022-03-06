package lib

import (
	"mate/pkg"
	"strconv"
)

// Calculator is token-compiler structure model.
// Takes a token array which will be compiled to [float32] answer.
type Calculator struct {
	Input []Token
}

// NewCalculator creates a new calculator structure.
// It's default way of creating calculators in the mate.
func NewCalculator(input []Token) Calculator {
	return Calculator{Input: input}
}

// Calculate, is token-to-number compiler of application.
// Loops through input and returns final answer.
// If there is an error, answer will be "ZERO", and error would be provided.
// The input argument can be passed from function arguments, if it's not provided
// from arguments, function uses default input of Calculator ──▶ [l.Input]
//
//  TODO: Add visual explanation.
//
func (c *Calculator) Calculate(input []Token) (float32, error) {
	var res float32

	var in []Token
	if input != nil {
		in = input
	} else {
		in = c.Input
	}

	// In case of having one but sub-expression token
	// We have to use its sub tokens to calculate.
	if len(in) == 1 && in[0].IsSubExp() {
		return c.Calculate(in[0].SubTokens)
	}

	for i := 0; i <= len(in); i += 2 {
		t := in[i]

		var operation TokenType
		var x, y float32 = res, 0

		// Check token validness.
		if t.IsIllegal() {
			err := []interface{}{i, t.Type, t.Literal, t.SubTokens}
			return 0, pkg.IllegalTokenError(err)
		}

		// Fill Y with number.
		if t.IsNum() {
			yRes, yErr := strconv.ParseFloat(t.Literal, 32)
			if yErr != nil {
				return 0, yErr
			}

			y = float32(yRes)
		} else if t.IsSubExp() {
			// Calculate inside portion of sub-expression.
			yRes, yErr := c.Calculate(t.SubTokens)
			if yErr != nil {
				return 0, yErr
			}

			y = float32(yRes)
		}

		// At first loop, operation must to be PLUS.
		// Because, res is zero and we have to
		// add some value before starting working on it.
		if i == 0 {
			operation = PLUS
		} else if in[i-1].IsPlusOrMinus() || in[i-1].IsProdOrDiv() {
			operation = in[i-1].Type
		} else {
			return 0, pkg.NoOperation([]interface{}{x, y})
		}

		// Update res by current X/Y/O.
		res = c.ExecuteOperation(x, y, operation)
	}

	return res, nil
}

// ExecuteOperation, executes operation for X and Y 
// numbers by appropriate operation type.
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
