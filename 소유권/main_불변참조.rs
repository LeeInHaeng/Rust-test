fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // & 기호를 사용하여 참조를 생성한다.
                                    // 참조는 소유권을 갖지 않기 때문에 참조가 가리키는 값은
                                    // 참조가 범위를 벗어나더라도 drop 함수가 호출되지 않는다.

    println!("'{}'의 길이는 {} 입니다.", s1, len);

    change(&s1);
}

fn calculate_length(s: &String) -> usize { // 매개변수 s는 String에 대한 참조다.
    s.len()
} // 이 시점에서 변수 s가 범위를 벗어난다.
  // 하지만 이 변수는 자신이 가리키는 값에 대한 소유권이 없으므로 아무 일도 일어나지 않는다.

fn change(some_string: &String) {
    some_string.push_str(", world!"); // 대여한 변수를 수정하려고 하면 컴파일 에러가 발생한다!
                                    // 변수가 불변인 것처럼 참조도 기본적으로 불변이다.
                                    // 따라서 참조하고 있는 값을 변경할 수 없다.
}