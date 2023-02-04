use std::net::TcpStream;
use std::io::{stdin, BufReader, BufRead, Write};
use std::thread;
use std::time::Duration;

fn main() {

    let server_addr = "127.0.0.1:8888";

    // 서버에 접속
    let mut socket = TcpStream::connect(server_addr).expect("서버에 접속 불가");
    socket.set_nonblocking(true).expect("unknown error");
    println!("{} 에 접속했습니다", server_addr);

    // 수신용 스레드 시작
    start_thread(socket.try_clone().unwrap());

    // 표준 입력으로 사용자 이름 지정
    let user = input("이름: ");
    println!("{} 님 메세지를 입력해주세요", user);

    loop {
        // 서버에 메세지 전달
        let msg = input(""); // 사용자 메세지 입력
        let msg = format!("{}> {}\n", user, msg);
        let buf = msg.as_bytes();
        socket.write_all(buf).unwrap();
    }
}

// 서버로부터의 메세지 수신 스레드
fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop{
        // 수신 대기
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            // 수신한 메세지를 화면에 표시
            if n > 0 { println!("[받은 메세지] {}", msg.trim()); }
        }

        thread::sleep(Duration::from_millis(100));
    });
}

fn input(msg: &str) -> String {
    if msg != "" { println!("{}", msg)}
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("입력 에러");
    String::from(buf.trim())
}

