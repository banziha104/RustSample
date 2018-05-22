# 템플릿 문법

- {} : 순서대로 출력
- {1},{2} : 매개변수 순서에 맞게 출력
- {named} : 이름에 맞는 파라미터를 가져옮
- {:\x\} : \x\에 맞게 출력
    - ? : 디버그
    - o : 8진법
    - b : 2진법
    - p : 포인터
    - e : LowerExp
    - E : UpperExp


```rust
format!("Hello");                 // => "Hello"
format!("Hello, {}!", "world");   // => "Hello, world!"
format!("The number is {}", 1);   // => "The number is 1"
format!("{:?}", (3, 4));          // => "(3, 4)"
format!("{value}", value=4);      // => "4"
format!("{} {}", 1, 2);           // => "1 2"
format!("{:04}", 42);             // => "0042" with leading zeros
```