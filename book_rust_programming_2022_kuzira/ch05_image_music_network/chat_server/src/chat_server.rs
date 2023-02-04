use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {

    let server_addr = "127.0.0.1:8888";

    let (tx, rx) = mpsc::channel::<String>();

    // 클라이언트 목록 저장용
    let mut clients: Vec<TcpStream> = Vec::new();

    // 서버 시작
    let server = TcpListener::bind(server_addr).expect("server start error");
    server.set_nonblocking(true).expect("unknown error");
    println!("{} 에서 서버가 실행중입니다", server_addr);

    // wait for clients
    loop {
        // 신규 클라이언트 등록 및 담당 스레드 생성
        if let Ok((client, addr)) = server.accept() {
            println!("클라이언트 접속: {}", addr);
            clients.push(client.try_clone().unwrap());

            start_thread(client, tx.clone());
        }

        // 클라이언트별로 메세지가 오면 이를 전원에게 전송
        if let Ok(msg) = rx.try_recv() {
            println!("전원에게 보내기: {}", msg.trim());
            clients = send_all(clients, &msg);
        }
    }
}

// 클라이언트가 보내는 메세지 수신 스레드
// 각 클라이언트에 대해 1개씩 생기며 클라이언트의 메세지를 main thread로 전달한다.
fn start_thread(client: TcpStream, tx:mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop{
        // 수신 대기
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            // 수신한 메세지를 메인 스레드로 전달
            if n > 0 { tx.send(msg).unwrap(); }
        }

        thread::sleep(Duration::from_millis(100));
    });
}

fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    // 소유권을 iter시에 move 하므로 다시 회수하여 리턴한다.
    let mut collector = vec![];

    for mut socket in clients.into_iter() { // 소유권이 있어야 쓰기를 할 수 있으므로 부득이하게 소유권을 넘겨준다.
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("전송 에러: {}", e);
            continue;
        }
        collector.push(socket); //  소유권 회수
    }
    collector // 소유권 반납
}



