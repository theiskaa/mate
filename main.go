package main

import (
	"mate/repl"
	"os"
)

func main() {
	// Start execution of repl.
	repl.Start(os.Stdin, os.Stdout)
}
