# 고차원 함수와 클루져

- 클로져 : 익명함수로 |n| { *logic* } 의 형식을 취함, 둘러싸는 변수에 대한 참조자만 필요
- 이동 클로져 : move 키워드로, 모든 변수의 소유권을 가져감, 주로 프로그램이 서로 동시적 스레드와 함께 동작할 떄 사용

```rust
fn again<F : Fn(i32) -> i32>(f: F, s:i32) -> i32 {
    f(f(s))
}

fn triples(s : i32) -> i32 { 3 * s}

fn main() {
    let mut strengh = 78;
//    let triples = |n|(triples,strengh);
    strengh = again(|n|{3 * n },strengh); // 익명함수 클로져
    
    let m : i32 = 42; // 이동 클로져는 둘러싸는 모든 변수의 소유권을 가져감
    let print_add_move = move |s| {
        println!("m is {}",m)
        m + s
    }
}
```