# 포인터와 메모리 안정성

### 포인터 

- &T : T를 읽는 하나 이상의 레퍼런스를 허용
- &mut T : T를 읽고 쓰는 하나의 레퍼런스를 허용
- Arc<T> : 소유자를 복수로 허용하며 여러 스레드에 걸쳐 안전한 Mutable 공유
- *const T L T의 안전하지 않은 읽기 접근 허용
- *mut T : T의 안전하지 않은 읽기/쓰기 접근을 허용

- Box<T> : 박스는 복사할 수 없는 값

```rust
let mut a1 = Box::new(Student{grade:10}) // 박스포인터
```

- Rc<T> : 여러 레퍼런스는 같은 리소스를 공유할 수 있음

```rust

struct Alien {
	planet: String,
	n_tentacles: u32,
}

fn main() {
	let mut klaatu = Alien{ planet: "Venus".to_string(), n_tentacles: 15 };

	{	
		let kl2 = &mut klaatu;
		kl2.n_tentacles = 14;
		println!("{} - {}", kl2.planet, kl2.n_tentacles); // Venus - 14
		kl2.n_tentacles = 10;
	}

	println!("{} - {}", klaatu.planet, klaatu.n_tentacles); // Venus - 10
	klaatu.planet = "Pluto".to_string();
	println!("{} - {}", klaatu.planet, klaatu.n_tentacles); // Pluto - 10
}

```





### 포인터와 레퍼런스

- 생존주기 : 변수가 생성되고, 참조되지 않을 때까지 유효한 것
- ' : 수명지정자로, 수명을 나타냄
- 포인터의 수명은 항상 포인터가 가리키는 값과 같거나 짧다.
- *const : 원시 포인터
- let a = &mut n : 변경 가능한 레퍼런스


### 값 복사와 복사 트레잇

- Copy : let a = c 와 같음
- Clone : let a = c.clone 과 같이 복사할수 있음


```rust
// 아래 두개는 다른 메모리 주소를 닽는다

#[derive(Copy)]
struct MagicNumber{
    value:u64
}

impl Copy for MagicNumber{

}

```

```rust
#[derive(Clone)]
struct Magic {
    value : u64
}

fn main() {
    let mut a = Magic{value:10};
    let mut b= a.clone();
    let c = a ;
    println!("{}",a.value);
    println!("{}",b.value);
//    println!("{}",c.value); Copy를 구현하지 않았음으로 사용 불가
    a.value = 15;
    println!("{}",a.value);
    println!("{}",b.value);
//    println!("{}",c.value);
}
```

### match 문 안에 레퍼런스

> 레퍼런스를 찾을 때 match 안에 ref 키워드를 이용해 찾을 수 있음.

```rust

struct Magician {
    name: &'static str,
    power: u32
}

fn main() {
    let n = 42;
    match n {
        ref r => println!("Got a reference to {}", r),
    }

    let mut m = 42;
    match m {
        ref mut mr => {
            println!("Got a mutable reference to {}", mr);
            *mr = 43;
        },
    }
    println!("m has changed to {}!", m);

    let mag = Magician { name: "Gandalf", power: 4625 };
    let name = {
        // `ref_to_x` is a reference to the `x` field of `point`
        let Magician { name: ref ref_to_name, power: _ } = mag;
        // Return a copy of the `name` field of `mag`
        *ref_to_name
    };
    println!("The magician's name is {}", name);
}
```

