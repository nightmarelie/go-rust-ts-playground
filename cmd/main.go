package main

import "fmt"

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
