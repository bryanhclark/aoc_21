fn main() {
  let lines = include_str!("input2.txt").lines();
  let mut forward = 0;
  let mut depth = 0;
  let mut aim = 0;
  for line in lines {
    let orders: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
    let dir = &orders[0 as usize];
    let val = &orders[1 as usize].to_string().parse::<i32>().unwrap();
    match dir.as_str() {
      "forward" => {
        forward += val;
        depth += aim * val;
      }
      "up" => {
        aim -= val;
      }
      "down" => {
        aim += val;
      }
      _ => println!("unmatched direction"),
    }
  }
  println!("FORWARD: {}", forward);
  println!("Depth: {}", depth);
  println!("Depth: {}", aim);
  println!("Answer: {}", depth * forward);
}
