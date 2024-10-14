package main

import (
	"fmt"
	"strconv"
	"strings"
)

func run_part1(input string) string {
	instructions := parseInstructions(input)
	area := getTrenchArea(instructions)
	return strconv.Itoa(int(area))
}

type Direction string

const (
	Up    Direction = "U"
	Down  Direction = "D"
	Right Direction = "R"
	Left  Direction = "L"
)

type Instruction struct {
	direction Direction
	length    uint
}

type Position [2]int

func parseInstruction(line string) *Instruction {
	splitted := strings.Split(line, " ")
	if len(splitted) != 3 {
		panic(fmt.Sprintf("Invalid input: %s", line))
	}
	direction := splitted[0]
	length, err := strconv.Atoi(splitted[1])
	if err != nil {
		panic(err)
	}
	return &Instruction{Direction(direction), uint(length)}
}

func parseInstructions(input string) []*Instruction {
	result := make([]*Instruction, 0)
	for _, line := range strings.Split(input, "\n") {
		result = append(result, parseInstruction(line))
	}
	return result
}

func getTrenchArea(instructions []*Instruction) uint {
	currentPosition := Position{0, 0}
	positions := make([]Position, 0)
	positions = append(positions, currentPosition)
	for _, instruction := range instructions {
		currentPosition = getNewPosition(&currentPosition, instruction)
		positions = append(positions, currentPosition)
	}
	// displayPositions(positions)
	return computeArea(positions) + computerPerimeter(positions)/2 + 1
}

func computerPerimeter(positions []Position) uint {
	var sum uint = 0
	for i, pos1 := range positions[:len(positions)-1] {
		pos2 := positions[i+1]
		sum += abs(pos1[0]-pos2[0]) + abs(pos1[1]-pos2[1])
	}
	return sum
}

func computeArea(positions []Position) uint {
	sum := 0
	for i, pos1 := range positions[:len(positions)-1] {
		pos2 := positions[i+1]
		sum += pos1[0]*pos2[1] - pos1[1]*pos2[0]
	}
	return abs(sum) / 2
}

func abs(val int) uint {
	if val < 0 {
		return uint(-val)
	}
	return uint(val)
}

func getNewPosition(current_position *Position, instruction *Instruction) Position {
	var newPosition Position
	switch instruction.direction {
	case Up:
		newPosition = Position{current_position[0] - int(instruction.length), current_position[1]}
	case Down:
		newPosition = Position{current_position[0] + int(instruction.length), current_position[1]}
	case Right:
		newPosition = Position{current_position[0], current_position[1] + int(instruction.length)}
	case Left:
		newPosition = Position{current_position[0], current_position[1] - int(instruction.length)}
	default:
		panic("Invalid direction")
	}
	return newPosition
}

func normalizePositions(positions []Position, minRow, minCol int) []Position {
	normalizedPositions := make([]Position, len(positions))
	for i, position := range positions {
		normalizedPositions[i] = Position{position[0] - minRow, position[1] - minCol}
	}
	return normalizedPositions
}

func getBounds(positions []Position) (int, int, int, int) {
	minRow := positions[0][0]
	minCol := positions[0][1]
	maxRow := positions[0][0]
	maxCol := positions[0][1]
	for _, position := range positions[1:] {
		minRow = min(minRow, position[0])
		minCol = min(minCol, position[1])
		maxRow = max(maxRow, position[0])
		maxCol = max(maxCol, position[1])
	}
	return minRow, minCol, maxRow + 1, maxCol + 1
}

func displayPositions(positions []Position) {
	println(len(positions))
	minRow, minCol, maxRow, maxCol := getBounds(positions)
	normalizedPositions := normalizePositions(positions, minRow, minCol)
	maxRow = maxRow - minRow
	maxCol = maxCol - minCol
	display := make([][]byte, maxRow)
	for i := range display {
		display[i] = make([]byte, maxCol)
		for j := range display[i] {
			display[i][j] = '.'
		}
	}
	for _, position := range normalizedPositions {
		display[position[0]][position[1]] = '#'
	}
	for _, row := range display {
		println(string(row))
	}
}

func min(a, b int) int {
	if a > b {
		return b
	}
	return a
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
