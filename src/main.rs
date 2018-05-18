
static mut A : i32 = 10; // 변경가능한 전역상수, 타입을 지정해주어야함

fn main(){
    let a = 5;
    let b = false;
    let empty = ();
    let mut n = 10; // 변경가능한상수

    {   // 스코프 별도 생성
        let c = 30;
    }

//    print!(c) 에러
}
