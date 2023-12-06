fn main() {
    // Time:        63     78     94     68
    // Distance:   411   1274   2047   1035

    let mut sol1: u64 = 1;
    sol1 *= sols(63, 411);
    sol1 *= sols(78, 1274);
    sol1 *= sols(94, 2047);
    sol1 *= sols(68, 1035);
    dbg!(sol1);

    let sol2 = sols(63789468, 411127420471035);
    dbg!(sol2);
}

fn sols(time: u64, distance: u64) -> u64 {
    let delta = time.pow(2) - 4 * distance;
    let min = ((time as f64 - (delta as f64).sqrt()) as f64 / 2.0 + 0.0001).ceil() as u64;
    let max = ((time as f64 + (delta as f64).sqrt()) as f64 / 2.0 - 0.0001).floor() as u64;
    max - min + 1
}
