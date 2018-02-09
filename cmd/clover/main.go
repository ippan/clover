package main

import (
	"io"
	"bufio"
	"fmt"
	"os"
	"github.com/ippan/clover/lexer"
	"github.com/ippan/clover/token"
)

const prompt = "clover>"

func start_repl(reader io.Reader) {
	scanner := bufio.NewScanner(reader)

	for {
		fmt.Print(prompt)

		scanner.Scan()

		line := scanner.Text()

		if line == "exit" {
			return
		}

		luck_lexer := lexer.New(line)

		for luck_token := luck_lexer.Lex(); luck_token.Type != token.EOF; luck_token = luck_lexer.Lex() {
			fmt.Printf("%+v\n", luck_token)
		}
	}

}

func main() {
	start_repl(os.Stdin)
}