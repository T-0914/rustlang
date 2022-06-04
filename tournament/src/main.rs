use tournament::tally;

fn main() {
    const SAMPLE_MATCHES: &str = "A;B;win\nC;D;draw\nC;A;win\nD;B;loss\nB;C;loss\nA;D;win";

    let report = tally(SAMPLE_MATCHES);
}
