package lib_test

import (
	"mate/lib"
	"testing"
)

func TestCalculatorsCalculate(t *testing.T) {
	tests := []struct {
		input, inArg  []lib.Token
		expected      float32
		expectedIsErr bool
	}{
		{
			expected: -7.5,
			input: []lib.Token{
				{Type: lib.NUMBER, Literal: "2"},
				{Type: lib.PLUS, Literal: "+"},
				{Type: lib.NUMBER, Literal: "-4.5"},
				{Type: lib.MINUS, Literal: "-"},
				{Type: lib.NUMBER, Literal: "5"},
			},
		},
		{
			expected: -18,
			inArg: []lib.Token{
				{Type: lib.SUB_EXP, SubTokens: []lib.Token{
					{Type: lib.SUB_EXP, SubTokens: []lib.Token{
						{Type: lib.SUB_EXP, SubTokens: []lib.Token{
							{Type: lib.NUMBER, Literal: "4"},
							{Type: lib.PRODUCT, Literal: "*"},
							{Type: lib.NUMBER, Literal: "5"},
						}},
						{Type: lib.MINUS, Literal: "-"},
						{Type: lib.NUMBER, Literal: "5"},
					}},
					{Type: lib.PRODUCT, Literal: "*"},
					{Type: lib.NUMBER, Literal: "-2"},
				}},
				{Type: lib.PLUS, Literal: "+"},
				{Type: lib.SUB_EXP, SubTokens: []lib.Token{
					{Type: lib.NUMBER, Literal: "24"},
					{Type: lib.DIVIDE, Literal: "/"},
					{Type: lib.NUMBER, Literal: "2"},
				}},
			},
		},
		{
			expected: 3,
			inArg: []lib.Token{
				{Type: lib.SUB_EXP, SubTokens: []lib.Token{
					{Type: lib.SUB_EXP, SubTokens: []lib.Token{
						{Type: lib.SUB_EXP, SubTokens: []lib.Token{
							{Type: lib.SUB_EXP, SubTokens: []lib.Token{
								{Type: lib.SUB_EXP, SubTokens: []lib.Token{
									{Type: lib.SUB_EXP, SubTokens: []lib.Token{
										{Type: lib.SUB_EXP, SubTokens: []lib.Token{
											{Type: lib.SUB_EXP, SubTokens: []lib.Token{
												{Type: lib.SUB_EXP, SubTokens: []lib.Token{
													{Type: lib.NUMBER, Literal: "12"},
													{Type: lib.PRODUCT, Literal: "*"},
													{Type: lib.NUMBER, Literal: "0.5"},
												}},
												{Type: lib.DIVIDE, Literal: ":"},
												{Type: lib.NUMBER, Literal: "2"},
											}},
										}},
									}},
								}},
							}},
						}},
					}},
				}},
			},
		},
		{
			expected:      0,
			expectedIsErr: true,
			input: []lib.Token{
				{Type: lib.SUB_EXP, SubTokens: []lib.Token{
					{Type: lib.NUMBER, Literal: "21"},
					{Type: lib.PLUS, Literal: "+"},
					{Type: lib.NUMBER, Literal: "21"},
				}},
				{Type: lib.NUMBER, Literal: "2"},
				{Type: lib.PLUS, Literal: "-"},
				{Type: lib.SUB_EXP, SubTokens: []lib.Token{
					{Type: lib.NUMBER, Literal: "21"},
					{Type: lib.PRODUCT, Literal: "*"},
					{Type: lib.NUMBER, Literal: "2"},
				}},
			},
		},
		{
			expected:      0,
			expectedIsErr: true,
			inArg: []lib.Token{
				{Type: lib.NUMBER, Literal: "2"},
				{Type: lib.PLUS, Literal: "-"},
				{Type: lib.ILLEGAL},
			},
		},
	}

	for _, td := range tests {
		c := lib.NewCalculator(td.input)
		got, gotErr := c.Calculate(td.inArg)

		if got != td.expected {
			t.Errorf("Sum was different of Calculate. Want: %v | Got: %v", td.expected, got)
		}

		if td.expectedIsErr == (gotErr == nil) {
			t.Errorf("Sum was different of Calculate's error | Got: %v", gotErr)
		}
	}
}

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
