// [ ] get number of digits
// [ ] for each digit, power it to the number of digits
// [ ] sum it up all these powered numbers
// [ ] return true if the sum is equal to the original number

pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
      .to_string() // converts to String
      .chars() // allows to iterate over each character
      .map(|c| // uses the previous iterator and for each element, executes a closure
        c.to_digit(10) 
        .unwrap_or(0)) // i only want the convertions that succeeded, otherwise, it should be 0
      .collect(); // from iter to collection (expects a type to be specified like in the line 7)

    let size = digits.len();

    let sum: u32 = digits.iter().map(|val| {
      val.pow(size as u32) // how can i do this usize to u32 using "into" ?
    }).sum();

    num == sum
}
