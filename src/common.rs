use rand::{thread_rng, Rng};

pub fn select_by_random<T: Clone>(array: &Vec<T>) -> T {
  let mut rng = thread_rng();

  array[rng.gen_range(0..array.len())].clone()
}