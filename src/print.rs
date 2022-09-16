pub fn run() {
    // Print to console
    println!("Print from print.rs");
    println!("{}", 1);
    println!("Number: {}!!!", 1);
    println!("First {}, second {}", "here", 2);
    println!("{0}-{1}-{0}", 9, 1);
    println!("{First}, {Last}", First = "1", Last = "one");
    println!("Binary: {:b} Hex: {:x} Octo {:o}", 10, 10, 10);
    println!("{:?}", (123, true, "what!"));
}
