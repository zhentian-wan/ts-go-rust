package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

func getInput() string {
  return `forward 5
down 5
forward 8 
up 3
down 8
forward 2`;
}


type Point struct {
	x int
	y int
}

func parseLine(line string) Point {
	parts := strings.Split(line, " ")
	amount, err := strconv.Atoi(parts[1])

	if err != nil {
		log.Fatal("this should never ever happen")
	}

	if parts[0] == "forward" {
		return Point {
			x: amount,
			y: 0,
		}
	} else if parts[0] == "up" {
		return Point {
			x: 0,
			y: -amount,
		}
	}

		return Point {
			x: 0,
			y: amount,
		}
}

func main() {
		lines := strings.Split(getInput(), "\n")

		pos := Point{0,0}
		for _, line := range lines {
			  p := parseLine(line)
				pos.x += p.x
				pos.y += p.y
		}

		value := fmt.Sprintf("point: %+v", pos)
		fmt.Println(value) 
}
