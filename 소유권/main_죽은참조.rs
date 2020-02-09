fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 함수는 String에 대한 참조를 리턴한다.
    let s = String::from("hello"); // String 타입의 새로운 변수 s를 생성한다.

    &s // String 타입의 변수 s에 대한 참조를 리턴한다.
        // 하지만 이 지점에서 변수 s가 범위를 벗어나므로 drop 함수가 호출되고 메모리가 해제된다.
        // 따라서 이 함수는 에러의 위험을 내포하고 있다.
    
    // 문제 해결을 위해서는 &String 반환을 String으로 바꾸고
    // s를 리턴하면 소유권이 함수를 호출한 코드로 이동하기 때문에 문제가 되지 않는다!
}