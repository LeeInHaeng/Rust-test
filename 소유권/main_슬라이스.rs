fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..];

    let word = first_word(&s);

    let s2 = "Hello world"; // s2 변수의 타입은 &str 이다!
}

fn first_word(s: &String) -> &str { // &str 은 문자열 슬라이스를 표현하는 타입이다.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}