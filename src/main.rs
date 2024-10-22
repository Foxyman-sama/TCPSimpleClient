// Написать клиента, устанавливающего соединение с сервисом.
// Реализовать ввод данных с клавиатуры, а так же вывод ответа с сервера на экран (консольное приложение).

use core::str;
use std::{
  env,
  io::{BufRead, BufReader, BufWriter, Read, Write},
  net::TcpStream,
};

fn main() {
  let mut stream = create_tcp_connection();

  loop {
    {
      write_request(&mut stream);
    }
    {
      read_responce(&stream);
    }
  }
}

fn create_tcp_connection() -> TcpStream {
  let args: Vec<String> = env::args().collect();
  let port = args.get(1).unwrap();
  let addr = format!("localhost:{}", port);
  TcpStream::connect(addr).unwrap()
}

fn write_request(stream: &mut TcpStream) {
  let mut buf_writer = BufWriter::new(stream);
  let _ = buf_writer.write_all(get_string_from_input().as_bytes());
}

fn get_string_from_input() -> String {
  let mut result = String::new();
  std::io::stdin().read_line(&mut result).unwrap();
  result.push('@');
  result
}

fn read_responce(stream: &TcpStream) {
  let mut buf_reader = BufReader::new(stream);
  let mut responce = vec![];
  let _ = buf_reader.read_until(b'@', &mut responce);
  let responce = str::from_utf8(&responce).unwrap();
  std::println!("{}", responce);
}
