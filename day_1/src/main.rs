mod read_file;



fn get_neighbor_sum(index: usize, arr: &Vec<String>) -> u32 {
  let first = arr[index - 1].to_string().parse::<u32>().unwrap();
  let second = arr[index ].to_string().parse::<u32>().unwrap();
  let third = arr[index + 1].to_string().parse::<u32>().unwrap();
  return first + second + third;

}

fn main() {
  // this is the solution to day1 pt 2
  let filename = "src/input2.txt";
  let lines = read_file::get_lines_from_file(filename);
  let lines_len = lines.len();
  let mut num_increases = 0;
  for i in 0..lines_len {
    // we dont want this to run on the first or last index
    if i <= 1 || i == lines_len - 1 {
      continue;
    }
    // sum groups of 3
    let prev_sum = get_neighbor_sum(i - 1, &lines);
    let curr_sum = get_neighbor_sum(i, &lines);
    if prev_sum < curr_sum {
      num_increases += 1;
    }
  }

    println!("{}", num_increases)
}
