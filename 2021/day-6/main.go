package main

import (
	"flag"
	"os"
	"strconv"
	"strings"
)

var part2 bool

func main() {
	flag.BoolVar(&part2, "part-2", false, "Part 2")
	flag.Parse()

	data, err := os.ReadFile(flag.Arg(0))
	if err != nil {
		panic(err)
	}

	values := strings.Split(string(data), ",")

	var initialFish []int
	for _, value := range values {
		parsedValue, err := strconv.Atoi(strings.TrimSpace(value))
		if err != nil {
			panic(err)
		}
		initialFish = append(initialFish, parsedValue)
	}

	var fishCount int
	if !part2 {
		fishCount = runPart1(initialFish)
	} else {
		fishCount = runPart2(initialFish)
	}
	println(fishCount)
}

func runPart1(fish []int) int {
	for day := 1; day <= 80; day++ {
		for idx := 0; idx < len(fish); idx++ {
			if fish[idx] == 0 {
				fish[idx] = 6
				fish = append(fish, 9)
			} else {
				fish[idx]--
			}

		}
	}

	return len(fish)
}

func runPart2(initialFish []int) int {
	var fish [9]int

	for _, value := range initialFish {
		fish[value]++
	}

	for day := 1; day <= 256; day++ {
		newFish := fish[0]

		fish[0] = fish[1]
		fish[1] = fish[2]
		fish[2] = fish[3]
		fish[3] = fish[4]
		fish[4] = fish[5]
		fish[5] = fish[6]
		fish[6] = fish[7]
		fish[7] = fish[8]

		fish[6] += newFish
		fish[8] = newFish
	}

	var total int
	for _, value := range fish {
		total += value
	}
	return total
}
