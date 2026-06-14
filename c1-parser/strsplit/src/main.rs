use string_split::StrSplit;

mod string_split;

fn main() {
    let text = String::from("sahil,loves,sonia");

    let split = StrSplit::new(&text, ",");


    for part in split {
        println!("{}", part);
    }

}