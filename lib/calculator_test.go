package lib_test

import (
	"mate/lib"
	"testing"
)

func TestExecuteOperation(t *testing.T) {
	type arguments struct {
		x, y      float32
		operation lib.TokenType
	}

	tests := []struct {
		args     arguments
		expected float32
	}{
		{
			expected: 36.2,
			args: arguments{
				x: 35, y: 1.2,
				operation: lib.PLUS,
			},
		},
		{
			expected: 6,
			args: arguments{
				x: 48, y: 42,
				operation: lib.MINUS,
			},
		},
		{
			expected: 5.5,
			args: arguments{
				x: 11, y: 2,
				operation: lib.DIVIDE,
			},
		},
		{
			expected: 42,
			args: arguments{
				x: 6, y: 7,
				operation: lib.PRODUCT,
			},
		},
	}

	for _, td := range tests {
		c := lib.NewCalculator([]lib.Token{})

		got := c.ExecuteOperation(td.args.x, td.args.y, td.args.operation)
		if got != td.expected {
			t.Errorf("Sum was different of ExecuteOperation. Want: %v | Got: %v", td.expected, got)
		}

	}
}
