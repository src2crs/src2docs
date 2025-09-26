package task_fib

import "fmt"

func ExampleFib() {
	fmt.Println(Fib(0))
	fmt.Println(Fib(1))
	fmt.Println(Fib(2))
	fmt.Println(Fib(3))
	fmt.Println(Fib(4))
	fmt.Println(Fib(5))
	fmt.Println(Fib(6))
	fmt.Println(Fib(7))

	// Output:
	// 0
	// 1
	// 1
	// 2
	// 3
	// 5
	// 8
	// 13
}
