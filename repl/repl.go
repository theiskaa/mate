package repl

// TODO: Improve repl inputs(ask for logging tokens)
// TODO: Improve repl outputs(append state-appropriate colors to logging data)

import (
	"bufio"
	"fmt"
	"io"
	"mate/lib"
)

// Default strings of repl.
const PROMPT = "> "
const LINE = "\n────────────────────────"

// Start, executes scanner to read user-prompt-inputs
// and then converts it to result, and in final step logs result.
func Start(in io.Reader, out io.Writer) {
	scanner := bufio.NewScanner(in)

	for {
		fmt.Printf(PROMPT)

		if scanned := scanner.Scan(); !scanned {
			return
		}

		input := scanner.Text()
		if len(input) == 0 {
			fmt.Println("--- Empty Input ---")
			continue
		}

		// Generate Tokens.
		lexer := lib.NewLexer(input)
		tokens := lexer.Lex()

		// Calculate result from tokens.
		calculator := lib.NewCalculator(tokens)
		result, err := calculator.Calculate(tokens)

		fmt.Printf(
			`
Result:  %v
Error:  %v
				`,
			result, err,
		)

		fmt.Println(LINE)

		for _, t := range tokens {
			fmt.Printf("%+v\n", t)
		}

		fmt.Printf("\n")
	}
}
