extern crate proc_macro;
use proc_macro::TokenStream;

const REPLACEMENTS: &[(&str, &str)] = &[
    ("сюда", "use"),
    ("база", "std"),
    ("бассейн32", "f32"),
    ("ПИ", "PI"),
    ("постоянства", "consts"),
    ("даю", "let"),
    ("петля", "loop"),
    ("матрас!", "vec!"),
    ("тута", "for"),
    ("будь", "as"),
    ("стонать!", "print!"),
    ("выдать", "flush"),
    ("молиться", "unwrap"),
    ("нука", "if"),
    ("ну", "fn"),
    ("неа", "else"),
    ("река", "thread"),
    ("спать", "sleep"),
    ("время", "time"),
    ("Длительность", "Duration"),
    ("из_миллисека", "from_millis"),
    ("постоянство", "const"),
    ("жижа", "usize"),
    ("главная", "main"),
    ("глас", "stdout"),
    ("даю", "let"),
    ("задом", "rev"),
    ("в_шагать", "into_iter"),
    ("шагать", "iter"),
    ("залутать", "collect"),
    ("стонатьля", "println"),
    ("связь", "io"),
    ("Писать", "Write"),
    ("в", "in"),
    ("грех", "sin"),
    ("Строка", "String"),
    ("тута", "for"),
    ("мутант", "mut"),
];

#[proc_macro]
pub fn русы(input: TokenStream) -> TokenStream {
    let mut code = input.to_string();

    for (src, dest) in REPLACEMENTS {
        code = code.replace(src, dest)
    }

    code.parse().unwrap()
}
