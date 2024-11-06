#[allow(unused)]
use std::thread;
use std::time::Duration;
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation_fn: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation_fn: T) -> Cacher<T> {
        Cacher {
            calculation_fn,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation_fn)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    // こうすることで、closureは一度呼び出されたら、その結果をletの変数にbindする。そして、もう一度呼び出されたら、中のコードを実行するのではなく、
    // bindされた結果だけを返す。従って、呼び出し回数を減らすことができる。
    // というわけではな **ない** -> Cacherという構造体にクロージャを持たせて、キャッシュ機構を備えさせる。
    if intensity < 25 {
        println!("Today do {} pushes", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            // 今日は休憩してください！水分補給を忘れずに！
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                // 今日は、{}分間走ってください！
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}
