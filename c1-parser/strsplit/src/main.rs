use string_split::StrSplit;

mod string_split;

fn main() {
    let text = String::from("sahil,loves,sonia");
    let delim = ",";

    let split = StrSplit::new(&text, ",");

    let ours: Vec<_> =
    StrSplit::new(&text, delim).collect();

    let std: Vec<_> =
        text.split(delim).collect();

    assert_eq!(ours, std);

    println!("ours: {:?}", ours);
    println!("std : {:?}", std);


}