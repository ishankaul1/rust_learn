/*
TODO (homework/practice):
1. Convert temperatures between Fahrenheit and Celsius.
2. Generate the nth Fibonacci number.
3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
   DEFERRED #3; think I wont gain much from more loops.
   Lets move on to the good stuff!
*/

fn convert_fh_to_c(fh_temp: f64) -> f64 {
    (fh_temp - 32.0) * (5.0 / 9.0)
}

fn test_fh_conversion() {
    println!("TEST 1: F to C Conversion");
    println!("{}", "-".repeat(15));
    let test_temps = [0.0, 32.0, 60.0, 70.0, 100.0];
    for fh_temp in test_temps {
        let converted = convert_fh_to_c(fh_temp);
        println!("{fh_temp} degrees F => {converted} degrees C");
    }
    println!();
}

fn fibonacci(n: i32) -> Result<i32, &'static str> {
    if n < 0 {
        Err("Cannot return fibonacci index < 0")
    } else {
        let n_converted = n as usize;

        let mut vec = vec![0, 1];

        // fib(2) requires length=3. if length was 2 we would crash
        // therefore, need to add while length <= n, or while length - 1 < n
        // while vec.length() < n
        while vec.len() <= n_converted {
            vec.push(vec[vec.len() - 1] + vec[vec.len() - 2]);
        }

        Ok(vec[n_converted])
    }
}

fn test_fib() {
    println!("TEST 2: Fibonacci");
    let test_ns = [0, 1, 2, 3, 4, 5, 20, 30];
    for n in test_ns {
        let fib_val = fibonacci(n).expect("Fibonacci calc failed");
        println!("Fib({n}) = {fib_val}");
    }
}

fn main() {
    println!("Running tests!");
    println!("{}\n", "=".repeat(20));

    test_fh_conversion();
    test_fib();
}
