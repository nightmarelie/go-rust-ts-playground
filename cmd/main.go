package main

type GoEnum = int

const (
	One   GoEnum = 1
	Two   GoEnum = 2
	Three GoEnum = 3
)

func main() {
	// print out enum values
	println(One)
	println(Two)
	println(Three)
}
