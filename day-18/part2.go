package main

import (
	"fmt"
	"strconv"
	"strings"
)

func run_part2(input string) string {
	instructions := parseInstructions2(input)
	area := getTrenchArea(instructions)
	return strconv.Itoa(int(area))
}

func parseInstruction2(line string) *Instruction {
	splitted := strings.Split(line, " ")
	if len(splitted) != 3 {
		panic(fmt.Sprintf("Invalid input: %s", line))
	}
	color := splitted[2]
	if len(color) != 9 {
		panic(fmt.Sprintf("Invalid input: %s", line))
	}
	var direction Direction
	switch color[7] {
	case '0':
		direction = Right
	case '1':
		direction = Down
	case '2':
		direction = Left
	case '3':
		direction = Up
	default:
		panic(fmt.Sprintf("Invalid direction: %c", color[6]))
	}
	length, err := strconv.ParseInt(color[2:7], 16, 0)
	if err != nil {
		panic(err)
	}
	return &Instruction{direction, uint(length)}
}

func parseInstructions2(input string) []*Instruction {
	result := make([]*Instruction, 0)
	for _, line := range strings.Split(input, "\n") {
		result = append(result, parseInstruction2(line))
	}
	return result
}
