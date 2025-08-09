package task_fib

// TASK: Implement the function below.
// NOTE: The implementation must be recursive.

// fib calculates the nth Fibonacci number recursively.
func Fib(n int) int {
	// hint: The Fibonacci sequence is defined as follows:
	// fib(0) = 0
	// fib(1) = 1
	// fib(n) = fib(n-1) + fib(n-2) for n > 1

	// begin:solution
	if n <= 1 {
		return n
	}
	return Fib(n-1) + Fib(n-2)
	// end:solution
	// placeholder:return 0;
}
