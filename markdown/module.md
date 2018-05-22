# 모듈과 크레이트

- 크레이트 : 패키지 또는 라이브러리에 해당하며, rustc는 단하나의 크레이트만을 컴파일함
- 모듈 : 스플릿팅의 단위, 크레이트 안에 포함된 단위 mod 키워드를 이용해 모듈정의

```rust
mod printer{
    pub fn print_this(){
        println!("good")
    }
}

fn main() {
    printer::print_this(); //접근
}
```

- 모듈가져오기 : use 키워드 사용
- 외부에 있는 크리에이터 사용 : extern crate
- 외부 라이브러리 사용 : Cargo.toml을 편집하고 cargo build 수 후 cargo update 수행