use rand::{thread_rng, Rng};

fn main() {
    // 1 ~ 100 のランダムな数値を生成
    let guess_number = thread_rng().gen_range(1..=100);
    println!("Guess a number between 1 and 100");

    loop {
        // ユーザーの入力を取得
        let input = input();

        // ユーザーの入力を数値に変換
        let input_number: i32 = input.parse().unwrap();

        // ユーザーの入力とランダムな数値を比較
        if input_number == guess_number {
            // ユーザーの入力がランダムな数値と一致した場合
            println!("You guessed correctly!");
            break;
        } else if input_number < guess_number {
            // ユーザーの入力がランダムな数値より小さい場合
            println!("You guessed too low!");
        } else if input_number > guess_number {
            // ユーザーの入力がランダムな数値より大きい場合
            println!("You guessed too high!");
        }
    }
    println!("The number was: {}", guess_number);
}

// ユーザーの入力を取得
fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // 改行を削除
    // "123\n" -> "123"
    input.trim().to_string()
}
