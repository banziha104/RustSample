# 데이터 구조화

### 문자열

> 러스트에서 문자열은 유니코드이며, 문자열 슬라이스는 불변

```rust
fn main(){

    let magican1 = "Merlin"; // 문자열 슬라이스는 불변이며 고정된 크기로 할당됌
    let magican2 : &'static str = "Gandalf"; // *'static은 문자열이 정적할당 되었음을 의미, 프로그램 종료시까지 남음
    let mut str = String::new(); // 빈문자열 생성
    let mut str2 = String::with_capacity(25); // 문자열을 25만큼만 생성
    let mut str3 = magican1.to_string();
    let sl1 = &str3; // 문자열 슬라이스를 만듬

    str.push('a'); // 한문자 추가
    str.push_str("dddd"); // 문자열 추가

    for c in magican1.chars(){ // 한문자씩 뽑아냄
        
    }
    for c in magican1.split(""){//문자열나누기

    }
}
```

<br>

### 배열 

```rust
fn main(){
    let array = ["lee","young","joon"]; //배열 선언, 고정된 크기를 가짐
    let mut array2 = ["lee","young","joon"]; // 변경가능한 배열

    println!("{}",array.len()); // 배열 길이

    for a in array.iter(){ // 배열순회
        println!("{}",a)
    }

}
```

### 벡터

> 임의의 타입을 가지는 가변 배열, 스택과 같음

```rust
fn main() {
    let mut numbers: Vec<&str> = vec!["1","2","34"]; //  벡터 선언
    let mut ids: Vec<i32> = Vec::with_capacity(25); // 길이가 정해진 배열
    
    numbers.push("55"); // 요소 한개를 끝에추가
    numbers.pop(); // 요소한개를 끝에서 제거
    
}
```

### 슬라이스

```rust
fn main(){
    let arr = ["1","2","3","4","5"];
    let slc = &arr[1..3];
    println!("{}",slc);
}
```

### 튜플

> 서로 다른 타입의 묶음

```rust
let thor = ("thor",true,300); // 튜플 생성
let (name, _, power) = thor; // 비구조화 할당
```

### 구조체




