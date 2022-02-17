use std::thread;
use std::time::Duration;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// Closure Type Inference and Annotation
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

// Storing Closures Using Generic Parameters and the Fn Traits
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    // let expensive_closure = |num: u32| -> u32 {
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

pub fn run() {
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;
    // generate_workout(simulated_user_specified_value, simulated_random_number);
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here {:?}", x); // The x value is moved into the closure, because we added the move keyword
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

// Most of the time when specifying one of the Fn trait bounds,
// you can start with Fn and the compiler will tell you
// if you need FnMut or FnOnce based on what happens in the closure body.
