package pkg_test

import (
	"mate/pkg"
	"testing"
)

func TestIsNumber(t *testing.T) {
	tests := []struct {
		char     byte
		expected bool
	}{
		{char: '-', expected: false},
		{char: '+', expected: false},
		{char: '0', expected: true},
		{char: '1', expected: true},
		{char: '2', expected: true},
		{char: '3', expected: true},
		{char: '4', expected: true},
		{char: '5', expected: true},
		{char: '6', expected: true},
		{char: '7', expected: true},
		{char: '8', expected: true},
		{char: '9', expected: true},
	}

	for _, td := range tests {
		got := pkg.IsNumber(td.char)

		if got != td.expected {
			t.Errorf("Sum of IsNumber was different, Want: %v, Got: %v", td.expected, got)
		}
	}
}

func TestIsPoint(t *testing.T) {
	tests := []struct {
		char     byte
		expected bool
	}{
		{char: '.', expected: true},
		{char: ',', expected: true},
		{char: '-', expected: false},
		{char: '+', expected: false},
		{char: '*', expected: false},
		{char: '/', expected: false},
		{char: '1', expected: false},
		{char: '5', expected: false},
	}

	for _, td := range tests {
		got := pkg.IsPoint(td.char)

		if got != td.expected {
			t.Errorf("Sum of IsNumber was different, Want: %v, Got: %v", td.expected, got)
		}
	}
}
