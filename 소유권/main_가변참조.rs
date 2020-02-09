fn main() {
    let mut s = String::from("hello"); // 가변 변수를 생성한다.

    change(&mut s); // 이후에 가변 참조를 생성한다.

    // 가변 참조의 제약조건
    let mut s2 = String::from("hello");

    let r1 = &mut s2; // 특정 범위 내의 특정 데이터에 대한 가변 참조는
    let r2 = &mut s2; // 오직 한 개만 존재해야 한다.
                    // 이를 통해 컴파일 시점에서 데이터 경합을 방지할 수 있다.
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!"); // 가변 참조이기 때문에 값 변경이 가능하다.
}