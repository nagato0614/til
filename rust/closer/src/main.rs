use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32
{
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


fn generate_workout(intensity: u32, random_number: u32)
{
    if intensity < 25
    {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );

        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            // 今日は休憩してください! 水分補給を忘れずに!
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                // 今日は、{}分間走ってください!
                "Today, run for {} minutes!", simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );

    let expensive_closure = | num| {
        println ! ("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
}
