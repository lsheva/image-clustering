#[test]
fn compare_hashes_test() {
  let res = clustering::compare_hashes(64, 128);
  println!("{}", res)
}
