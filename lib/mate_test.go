package lib_test

import (
	"mate/lib"
	"testing"
)

func TestCalculate(t *testing.T) {
	tests := []struct {
		input    string
		expected float32
		isErr    bool
	}{
		{
			input:    "2 + -4.5 - 5",
			expected: 2 + -4.5 - 5,
			isErr:    false,
		},
		{
			input:    ".5 * .5 + 1",
			expected: .5*.5 + 1,
			isErr:    false,
		},
		{
			input:    "(4 * 5 - 5) * -2 + 24 / 2",
			expected: (4*5-5)*-2 + 24/2,
			isErr:    false,
		},
		{
			input:    "(24 / 4) + 2 - (-2 * -5)",
			expected: (24 / 4) + 2 - (-2 * -5),
			isErr:    false,
		},
		{
			input:    "&!@$!@$",
			expected: 0,
			isErr:    true,
		},
		{
			input:    "(42/5) * 1 / .5 & # %",
			expected: 0,
			isErr:    true,
		},
	}

	for _, td := range tests {
		got, gotErr := lib.Calculate(td.input)

		if got != td.expected {
			t.Errorf("Sum was different of Calculate. Want: %v | Got: %v", td.expected, got)
		}

		if td.isErr == (gotErr == nil) {
			t.Errorf("Sum was different of Calculate's error | Got: %v", gotErr)
		}
	}
}
