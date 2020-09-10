const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32 = 1;

fn main() {
    let (missiles, ready) = (STARTING_MISSLES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missles left", missiles - ready);
}
