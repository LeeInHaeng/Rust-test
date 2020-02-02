# rust 윈도우 설치
- rust url : https://www.rust-lang.org/tools/install
- LLDB url (디버깅) : http://releases.llvm.org/download.html#3.8.0
- Visual studio 2019 설치 : C++를 사용한 데스크톱 개발
- Python 3.6.X 설치 : 3.7 ~ 이후 버전은 현재 LLDB에서 에러 발생 (codelldb requires python)
- VS Code Extensions
  - Rust(rls)
  - CodeLLDB
- Rust 프로젝트 수행
```
$ cargo new hello_world --bin
$ code ./hello_world
```