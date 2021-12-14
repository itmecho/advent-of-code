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

	data, err := os.ReadFile(os.Args[1])
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

	fishCount := runPart1(initialFish, 80)
	println(fishCount)
}

func runPart1(fish []int, days int) int {
	for day := 1; day <= days; day++ {
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
