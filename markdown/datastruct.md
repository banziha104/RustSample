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

> 임의의 타입을 가지는 가변 배열, 스택에 배열을 더한 느낌

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

```rust
struct Kilograms{
    name : &'static str,
    health : i32,
    level: u8
}

fn main(){
    let mut kilo = Kilograms {name : "lee", health : 100, level: 2};
    kilo.level; // 구조체 변수 사용
    let Kilograms {name, health ,..} = kilo; // 비구조화 할당
    println!("{} / {}",name,health)
}

```

- 구조체에 메서드를 정의

```rust
struct Student{
    name : &'static str,
    age : i32
}

impl Student {
    fn new(name :&'static str, age : i32) -> Student{
        Student{name,age }
    }
    fn get_name(&self) -> &'static str{
        self.name
    }
    fn get_age(&self) -> i32{
        self.age
    }
}

fn main() {
    let a = Student::new("dd",21);
    println!("{},{}",a.get_name(),a.get_age());
    println!("{},{}",Student::get_age(&a),Student::get_name(&a));

}
```

### 열거체

```rust
enum Compass {
    North,South,East,West
}


fn main(){
    let direction = Compass::North;
}
```

### Result & Option

> 결과는 표준라이브러리에 정의된 열거체의 특별한 종류. 결과는 뭔가가 실행될때마다 Ok,Err 중하나가 됌. <br>
> 옵션은 또 다른 특별한 열거체로,  값이 존재하는 경우 Some, 값이 없는 경우는 None

```rust
use std::io;

fn main() {
    //ok는 결과값을 옵션 값으로반환하고, expect는 오류가 발생했을떄 값을 제공하거나 메세지를 보여줌
    io::stdin()
            .read_line(&mut buf)
            .ok()
            .expect("Error!");
    
}
```

