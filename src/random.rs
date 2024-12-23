use chrono::Utc;

pub fn singl_number() -> Option<u8> {
    let last = Utc::now().time().to_string().chars().last()?;
    Some(last.to_string().trim().parse().unwrap())
}

#[test]
fn single_number_counter() {
    let mut list = [0; 10];
    for _ in 0..100 {
        if let Some(i) = singl_number() {
            list[i as usize] += 1;
        }
    }
    println!("{:?}", list);
}
