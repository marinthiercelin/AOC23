package main

import (
	"os"
)

func main() {
	if len(os.Args) != 2 {
		panic("Expected the path to the input")
	}
	filename := os.Args[1]
	input, err := os.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	println(string(input))
	output := run_part2(string(input))
	println(output)
}
