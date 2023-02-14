use std::io::{ErrorKind};
use std::io::{Write, stderr};
use std::path::Path;

struct WeatherReport {
    msg: String
}

/// 함수 시그니쳐에 Result<T, E> 를 정의함으로써 에러가 발생 가능한 함수임을 명시한다. 
/// 발생 가능한 에러의 종류도 명시한다. 
/// 구현에서는 어떤 경우에 Ok()가 발생하고 어떤 경우에 Err 이 발생하는지를 구분한다.
fn get_weather(location: &str) -> Result<WeatherReport, std::io::Error> {
    match location {
        "seoul" => Ok(WeatherReport {msg: "shiny".to_string() }),
        _ => Err(std::io::Error::new(ErrorKind::Other, "oh no!")),
    }    
}



/// 오류 메시지를 stderr에 덤프한다. 원인을 쫓아가면서 모두 출력
fn print_error(mut err: &dyn std::error::Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "\t casused by: {}", source);
        err = source;
    }
}

fn main() {

    let location = "inchon";
    let result = get_weather(location);
    match result {
        Ok(wr) => {
            println!("{}'s weather is {}", location, wr.msg);
        },
        Err(err) => {
            //println!("error querying the weather: {}", err);  <-- 에러 그 자체만 출력
            print_error(&err);  // 에러의 원인을 계속 쫓아가며서 출력
        }
    }


    // OK 여부 boolean
    let is_ok = get_weather(location).is_ok();
    let is_err = get_weather(location).is_err();

    // error 발생시 Option<E>로 변형해서 처리, error가 없다면 None
    let err_to_opt = get_weather(location).err();
    match err_to_opt {
        Some(err) => println!("{}", err),
        None => println!("do sometning"),
    }

    // unwrap으로 Result<T, E> 에서 T를 얻는다.
    let weather = get_weather(location).unwrap();  

    // unwrap_or(fallback) 은 E 발생시 fallback을 대신 준다.
    let weather = get_weather(location).unwrap_or(WeatherReport { msg: "default".to_string() });

    // unwrap_pr_else(fallback_fn)은 E 발생시 클로져 실행해서 default value를 얻는다. 
    let default_weather_closure = |err:std::io::Error| WeatherReport { msg: "default".to_string() };
    let weather = get_weather(location).unwrap_or_else(default_weather_closure);

    // E 발생시 panic 발생
    let weather = get_weather(location).expect("cannot get weather"); 

    // Reference로 결과 얻기 (consume이 아님)
    let weatherRef = get_weather(location).as_ref();
    let weatherMutRef = get_weather(location).as_mut();




}

fn my_main() {
    let src_path = Path::new("srcPath");
    let dst_path = Path::new("dstPath");
    
    // main 에서는 오류 대응해야~
    match move_all(&src_path, &dst_path) {
        Ok(v) => println!("OK!!"),
        Err(err) => println!("{}", err),
    }

    // 애써 에러 무시하기
    let _ = move_all(&src_path, &dst_path);
}

fn my_main2() -> Result<(), std::io::Error> {
    let src_path = Path::new("srcPath");
    let dst_path = Path::new("dstPath");
    
    // main 은 누구한테 더이상 책임을 전가할 수 없으므로 어찌되었건 처리해야한다. 
    move_all(&src_path, &dst_path).expect("error");

    // main의 signature가 아무것도 반환하지 않는 경우 ?를 사용하면 아래처럼 반환자 형식이 Result가 아니라고 오류
    //     the `?` operator can only be used in a function that returns `Result` 
    // 반면에 Result를 반환하게 시그니쳐를 고치면 main도 오류 전파가 가능하다.
    move_all(&src_path, &dst_path)?;

    Ok(())
}


fn move_all(src: &Path, dst: &Path) -> std::io::Result<()> {  // Result<()> 는 Result<(), std::io::Error>의 축약형
    
    // Result<ReadDir,E>에서 ?을 통해 ReadDir 추출, E 이면 오류 전파
    let read_iter = src.read_dir()?;   // Iterator over the entries in a directory.

    for entry_result in read_iter {
        
        // Result<DirEntry, E> 에서 ?을 통해 DirEntry 추출, E 이면 오류 전파
        let entry = entry_result?;
        
        let dst_file = dst.join(entry.file_name());
        
        // Result<(), E> 에서 ?을 통해 () 추출, E 이면 오류 전파
        std::fs::rename(entry.path(), dst_file)?;
    }

    Ok(())
}

use std::io::BufRead;

fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, std::io::Error> {

    let mut numbers: Vec<i64> = vec![];

    for line_result in file.lines() {
        let line = line_result?;  // 줄을 읽을 때 std::io::Error 발생 가능
        let parsed = line.parse()?;  // 정수를 파싱할 때 std::num::ParseIntError 발생 가능 

        // 두개의 error가 서로 호환이 안되므로 아래처럼 컴파일 에러 발생 
        // the trait `From<ParseIntError>` is not implemented for `std::io::Error

        numbers.push(parsed);
    }

    Ok(numbers)

}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn read_numbers2(file: &mut dyn BufRead) -> Result<Vec<i64>, GenericError> {

    let mut numbers: Vec<i64> = vec![];

    for line_result in file.lines() {

        // GenericError 를 사용해서 두 에러 유형을 통합할 수 있다.
        let line = line_result?;  // 줄을 읽을 때 std::io::Error 발생 가능
        let parsed = line.parse()?;  // 정수를 파싱할 때 std::num::ParseIntError 발생 가능 

        numbers.push(parsed);
    }

    Ok(numbers)
}

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

fn parse_json() -> Result<(), JsonError> {

    Err(JsonError {
        message: "malformed json".to_string(),
        line: 12,
        column: 25,
    })
}

use std::fmt;
use std::fmt::Formatter;
impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}, line:col({}:{}", self.message, self.line, self.column)
    }
}

fn main3() {
    match parse_json() {
        Ok(_) => println!("Ok"),
        Err(err) => println!("{}", err),  // print 하려면 Display trait이 정의되어 있어야 한다.
    }
}



