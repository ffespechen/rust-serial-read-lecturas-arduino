
use serial2::SerialPort;

fn main() {
    let port = SerialPort::open("/dev/ttyACM1", 57600)
            .expect("No se pudo abrir el puerto");

    let mut buffer = [0; 4];

    loop {
        match port.read(&mut buffer) {
            Ok(t) => {
                for i in 0..t {
                    print!("{}", buffer[i] as char);
                }
            },
            Err(e) => println!("Error {}", e)
        }
    }
}
