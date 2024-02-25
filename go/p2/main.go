package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

type Point struct {
	x int
	y int
}

type Line struct {
	p1 *Point
	p2 *Point
}

func getInput() string {
  return `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`;
}

func parsePoint(line string) (*Point, error) {
	ps := strings.Split(line, ",");
	x, err := strconv.Atoi(ps[0]);
	if err != nil {
		return nil, err
	}

	y, err := strconv.Atoi(ps[1]);
	if err != nil {
		return nil, err
	}

	return &Point{x, y}, nil
}

func parseLine(line string) (*Line, error) {
	l := strings.Split(line, " -> ")
	p1, err := parsePoint(l[0])
	if err != nil {
		return nil, err
	}

	p2, err := parsePoint(l[1])
		if err != nil {
		return nil, err
	}

	return &Line {
		p1: p1, 
		p2: p2,
	}, nil
}

func isHOrV (line Line) bool {
	return line.p1.x == line.p2.x || line.p1.y == line.p2.y
}

func main() {
	lines := []Line {}
	for _, l := range strings.Split(getInput(), "\n") {
		line, err := parseLine(l)
		if err != nil {
			log.Fatal("Cannot parse the line")
		}
		if isHOrV(*line) {
			lines = append(lines, *line)
		}
	}

  // Print each line with actual values of points
  for _, line := range lines {
    fmt.Printf("Line from (%d,%d) to (%d,%d)\n", line.p1.x, line.p1.y, line.p2.x, line.p2.y)
  }
}