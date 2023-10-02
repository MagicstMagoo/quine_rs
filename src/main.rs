fn main() {
    let x = "fn main() {\n    let x = ";
    let y = ";\n    print!(\"{}{:?};\\n    let y = {:?};\\n{}\\n\\\"hello quine\\\"\", x, x, y, y)\n}";
    print!("{}{:?};\n    let y = {:?};\n{}\n\"hello quine\"", x, x, y, y)
}
