use string_split::StrSplit;

mod string_split;

fn main() {
    let cases = vec![
        ("a,b,c", ","),
        ("a,,b", ","),
        (",a", ","),
        ("a,", ","),
        ("", ","),
        ("a::b::c", "::"),
    ];

    for (text, delim) in cases {
        let ours: Vec<_> =
            StrSplit::new(text, delim).collect();

        let std: Vec<_> =
            text.split(delim).collect();

        assert_eq!(ours, std);

        println!("Input : {:?}", text);
        println!("Delim : {:?}", delim);
        println!("Ours  : {:?}", ours);
        println!("Std   : {:?}", std);
        println!("-------------------------");
    }

    println!("✅ All tests passed!");
}