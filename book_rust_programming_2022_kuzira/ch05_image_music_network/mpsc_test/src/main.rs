use std::sync::mpsc;  // multi-producer, single consumer
use std::thread;
use std::time::Duration;

// 1초마다 정해진 메세지를 보내고, 마지막에 quit
fn sleep_sender(name: &str, sender: mpsc::Sender<String>) {
    let msgs = ["큰고래", "흑동고래", "향유고래", "남방큰돌고래", "북극고래"];
    for msg in msgs {
        let msg = format!("{}: {}", name, msg);
        sender.send(msg).unwrap(); 
        thread::sleep(Duration::from_millis(1000));
    }
    sender.send("quit".to_string()).unwrap();
}

fn main() {

    // 쓰레드 간의 통신용 채널
    let (tx, rx) = mpsc::channel::<String>();

    //  Thread #1
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("우영우", sender)
    });

    //  Thread #2
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("이준호", sender)
    });

    // main loop 에서 receiving
    loop {
        let buf = rx.recv().unwrap();
        println!("[수신]{}", buf);
        if buf == "quit" { break; }
    }
    
}
