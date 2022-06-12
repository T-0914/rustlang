use tournament::tally;

fn main() {
    // const SAMPLE_MATCHES: &str = "A;B;win\nC;D;draw\nC;A;win\nD;B;loss\nB;C;loss\nA;D;win";
    // const SAMPLE_MATCHES: &str = "";
    const SAMPLE_MATCHES: &str = "A;B;loss\nA;B;win\nC;D;loss\nC;D;win";

    let report = tally(SAMPLE_MATCHES);
    // println!("report: {:#?}", report);
}
