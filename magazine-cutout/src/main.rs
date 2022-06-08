use magazine_cutout::can_construct_note;

fn main() {
    let magazine = "Enough is enough when enough is enough"
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "enough is enough".split_whitespace().collect::<Vec<&str>>();
    can_construct_note(&magazine, &note);
    println!(
        "can_construct_note(&magazine, &note): {:#?}",
        can_construct_note(&magazine, &note)
    );
}
