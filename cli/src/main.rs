fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path = (&args).get(1).unwrap(); // 入力されるアセンブラ言語のパス
    let output_path = (&args).get(1).unwrap(); // アセンブラ言語から生成された機械語を生成するパス
    dbg!(input_path);
    dbg!(output_path);
}
