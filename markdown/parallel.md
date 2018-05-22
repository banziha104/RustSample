# 동시성과 병렬성

- 스레드 만들기 : std::thread::spawn을 통해 생성
```rust
use std::thread;

fn main() {
    thread::spawn(move || {
        println!("Hello!!!!!!!!!!!!!!!!!!");
    }).join();
}
```

- join() : 부모 스레드를 막고 자식 스레드가 실행을 마칠때까지 기다림
- sleep_ms() : 지연
- unwrap() : Ok로 부터 값을 가져와 자식스레드의 결과를 반환하거나 , Err의 경우 패닉을 이르킴
- 최대 스레드 만들기


```toml
[dependencies]
num_cpus = "*" # 가용한 CPU를 모두가져옮

```

```rust

extern crate num_cpus;
extern crate threadpool;

use std::thread;
use threadpool::ThreadPool;

fn main() {
    let ncpus = num_cpus::get();
    println!("The number of cpus in this machine is: {}", ncpus);

    let pool = ThreadPool::new(ncpus);

    for i in 0..ncpus {
    	pool.execute(move || {
        	println!("this is thread number {}", i)
        });
    }

    thread::sleep_ms(50);
}

```

- 스레드 안전성 : 데이터 레이싱을 막기 위해 소유권 전략을 이용

```rust

```