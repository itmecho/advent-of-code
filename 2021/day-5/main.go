package main

import (
	"bufio"
	"flag"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

type Point struct {
	x int
	y int
}

type Line struct {
	start  Point
	finish Point
}

var includeDiagonals bool

func main() {
	flag.BoolVar(&includeDiagonals, "part-2", false, "Include diagonal lines")
	flag.Parse()

	lines, err := parseInput(flag.Arg(0))
	if err != nil {
		log.Fatalf("Failed to parse input: %s", err)
	}

	pointMap := make(map[Point]int)

	for _, l := range lines {
		if !includeDiagonals && l.start.x != l.finish.x && l.start.y != l.finish.y {
			continue
		}
		drawLine(l, pointMap)
	}

	var total int
	for _, count := range pointMap {
		if count > 1 {
			total++
		}
	}

	println(total)
}

func parseInput(filename string) ([]Line, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, wrap(err, "failed to open file %s", filename)
	}

	scanner := bufio.NewScanner(f)

	var lines []Line

	pattern := regexp.MustCompile(`(\d+),(\d+)\s*\->\s*(\d+),(\d+)`)

	for scanner.Scan() {
		matches := pattern.FindStringSubmatch(scanner.Text())
		if len(matches) != 5 {
			return nil, fmt.Errorf("input line didn't match regex: %s", scanner.Text())
		}

		var intMatches []int
		for _, match := range matches[1:] {
			n, err := strconv.Atoi(match)
			if err != nil {
				return nil, wrap(err, "failed to parse input as int")
			}

			intMatches = append(intMatches, n)
		}

		startPoint := Point{
			x: intMatches[0],
			y: intMatches[1],
		}
		finishPoint := Point{
			x: intMatches[2],
			y: intMatches[3],
		}

		lines = append(lines, Line{
			start:  startPoint,
			finish: finishPoint,
		})
	}

	return lines, nil
}

func drawLine(line Line, pointMap map[Point]int) {
	xInc := 0
	yInc := 0

	if line.start.x < line.finish.x {
		xInc = 1
	} else if line.start.x > line.finish.x {
		xInc = -1
	}
	if line.start.y < line.finish.y {
		yInc = 1
	} else if line.start.y > line.finish.y {
		yInc = -1
	}

	currentPoint := line.start

	for {
		pointMap[currentPoint] += 1
		if currentPoint.x == line.finish.x && currentPoint.y == line.finish.y {
			break
		}

		currentPoint.x += 1 * xInc
		currentPoint.y += 1 * yInc
	}
}

func wrap(err error, msg string, args ...interface{}) error {
	if err == nil {
		return err
	}

	filledMessage := fmt.Sprintf(msg, args...)

	return fmt.Errorf("%s: %w", filledMessage, err)
}
