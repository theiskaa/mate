package repl

import (
	"bufio"
	"fmt"
	"io"
	"mate/lib"
)

// Default prompt of repl.
const PROMPT = "> "

// Start, executes scanner to read user-prompt-inputs
// and then converts it to result, and in final step logs result.
func Start(in io.Reader, out io.Writer) {
	scanner := bufio.NewScanner(in)

	for {
		fmt.Printf(PROMPT)

		if scanned := scanner.Scan(); !scanned {
			return
		}

		lexer := lib.NewLexer(scanner.Text())
		tokens := lexer.Lex()

		for _, t := range tokens {
			fmt.Printf("%+v\n", t)
		}

		fmt.Printf("\n")
	}
}
