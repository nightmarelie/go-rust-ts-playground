package main

import "fmt"

type GoEnum = int

const (
	Foo1 GoEnum = iota
	Bar1
	Baz1
)

func returnsError(value int) error {
	return fmt.Errorf("error: %d", value)
}

type Foo struct{}

func (f Foo) returnsError(value int) error {
	return fmt.Errorf("error: %d", value)
}

func main() {
	err := returnsError(1)
	if err != nil {
		fmt.Println(err)
	}

	f := Foo{}
	err = f.returnsError(2)
	if err != nil {
		fmt.Println(err)
	}
} 
