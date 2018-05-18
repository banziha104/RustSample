# 함수

- 기본적으로 스네이크 케이스를 따름
- 함수 문서화 -> 러스트는 함수를 자동으로 문서화해줌 rustdoc 파일명.rs를 할경우 자동으로 생성
- 테스트 하고 싶은 함수는 #[test] 속성을 달아줌


```rust
#[warn(unused_valiables)] // 속성을 뜻하며, 어노테이션과 같음
fn increment_num(num: i32) -> f32 {
    (num + 1) as f32 // 세미콜론이 없으면 리턴 키워드없이 리턴가능
}

#[test] // 테스트 가능한 속성
#[cfg(target_os ="macos")]
fn on_mac(){ //맥에서만실행된다

} 

```