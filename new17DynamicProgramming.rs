// In Rust, we use a struct to hold the "global" state (the fib array)
// that was a static field in your Java class.
struct DynamicProgramming {
    fib: Vec<i32>,
}

impl DynamicProgramming {
    // Initialize the struct with the vector size
    fn new(n: usize) -> Self {
        DynamicProgramming {
            fib: vec![-1; n + 1],
        }
    }

    // Corresponds to your Java: public static int fibonacci(int n)
    // We use &mut self to access and modify the 'fib' vector stored in the struct
    fn fibonacci(&mut self, n: usize) -> i32 {
        if n <= 1 {
            return n as i32;
        }

        if self.fib[n] != -1 {
            return self.fib[n];
        }

        let res = self.fibonacci(n - 1) + self.fibonacci(n - 2);
        self.fib[n] = res;
        res
    }
}

// Corresponds to your Java: public static int fibonacci2(int n, int[] memo)
// In Rust, we pass a mutable slice (&mut [i32]) to represent the array
fn fibonacci2(n: usize, memo: &mut [i32]) -> i32 {
    if n <= 1 {
        return n as i32;
    }

    if memo[n] != -1 {
        return memo[n];
    }

    let res = fibonacci2(n - 1, memo) + fibonacci2(n - 2, memo);
    memo[n] = res;
    res
}

// Corresponds to your Java: public static void TabulationFibonacci(int n)
fn tabulation_fibonacci(n: usize) {
    if n == 0 {
        println!("Fibonacci of 0 is: 0");
        return;
    }

    // Create a vector of size n + 1
    let mut fib = vec![0; n + 1];
    fib[0] = 0;
    fib[1] = 1;

    for i in 2..=n {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    println!("Fibonacci of {} is: {}", n, fib[n]);
}

fn main() {
    let n = 6;

    // --- Approach 1: Struct (Replaces static field) ---
    // We create an instance to hold the `fib` array state
    let mut dp = DynamicProgramming::new(n); 
    println!("Fibonacci of {} is: {}", n, dp.fibonacci(n));

    // --- Approach 2: Passing the Memo Array ---
    // Initialize vector with -1
    let mut memo = vec![-1; n + 1];
    println!("Fibonacci of {} is: {}", n, fibonacci2(n, &mut memo));

    // --- Approach 3: Tabulation ---
    tabulation_fibonacci(n);
}
