package main

import (
	"bufio"
	"fmt"
	"github.com/ippan/clover/lexer"
	"github.com/ippan/clover/parser"
	"github.com/ippan/clover/runtime"
	"io"
	"os"
)

const prompt = "clover>"

func startRepl(reader io.Reader) {
	scanner := bufio.NewScanner(reader)
	r := runtime.New()

	for {
		fmt.Print(prompt)

		scanner.Scan()

		line := scanner.Text()

		if line == "exit" {
			return
		}

		l := lexer.New(line)
		p := parser.New(l)

		program := p.Parse()

		if len(p.Errors()) > 0 {
			for _, error := range p.Errors() {
				fmt.Printf("%s\n", error)
			}
			continue
		}

		result := r.Eval(program)

		if result == nil {
			// TODO : print errors
			continue
		}

		fmt.Printf("%s\n", result.Inspect())
	}

}

func main() {
	startRepl(os.Stdin)
}
