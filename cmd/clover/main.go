package main

import (
	"bufio"
	"fmt"
	"github.com/ippan/clover/lexer"
	"github.com/ippan/clover/parser"
	"io"
	"os"
)

const prompt = "clover>"

func startRepl(reader io.Reader) {
	scanner := bufio.NewScanner(reader)

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

		fmt.Print(program.String())
	}

}

func main() {
	startRepl(os.Stdin)
}
