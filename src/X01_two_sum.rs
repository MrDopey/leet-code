// https://leetcode.com/problems/two-sum/

pub fn run() {
  let nums = vec![2,7,11,15];

  let target = 9;
  let res = two_sums(nums, target);
  println!("{:?}", res);
}

fn two_sums(nums: Vec<i32>, target: i32) -> Vec<i32> {

  let a = (0..nums.len()).flat_map(|f| ((f+1)..nums.len()).map(move|g| (f, g) ) );
  // println!("{a:?}");

  for (first, second) in a{
    let &x = nums.get(first).expect("msg");
    let &y = nums.get(second).expect("msg");
    if x + y == target {
      return vec![first as i32, second as i32];
    }
  }

  panic!("help!!");
}

fn check(left: i32, right: i32, target: i32) -> bool {
    left + right == target
} 