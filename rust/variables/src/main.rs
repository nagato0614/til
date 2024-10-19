fn f_to_c(f: f64) -> f64
{
    let c = (5. / 9.) * (f - 32.);

    c
}

fn c_to_f(c: f64) -> f64
{
    let f = (9. / 5.) * c + 32.;

    f
}

fn fibonacci(n: u32) -> u32
{
    if n <= 1
    {
        1
    }
    else
    {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    // 華氏から摂氏に変換する
    let f = 10.;
    let c = 10.;

    println!("f : {}", c_to_f(c));
    println!("c : {}", f_to_c(f));

    // フィボナッチ数列
    println!("fibonacci : {}", fibonacci(10));
}