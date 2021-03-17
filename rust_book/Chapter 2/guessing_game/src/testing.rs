use rand::{thread_rng, Rng};

fn main(){
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0, 10);
    println!("{}", n);
    let m: f64 = rng.gen_range(-40.0f64, 1.3e5f64);
    println!("{}", m);

}