
extern crate regex;
extern crate rand;
extern crate byteorder;
extern crate rsntp;
extern crate chrono;

use std::net::{ToSocketAddrs, UdpSocket};
use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;
use std::io::Cursor;
use regex::Regex;
use rand::Rng;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use rsntp::SntpClient;
use std::thread;
use std::sync::{Arc, Mutex};


fn main() {
    println!("Hello, world!");
        let offset_result = NtpOffsetFetcher::fetch_ntp_offset_in_milliseconds(NtpOffsetFetcher::DEFAULT_NTP_SERVER);
        match offset_result {
            Ok(offset) => println!("NTP offset: {} ms", offset),
            Err(e) => eprintln!("Error fetching NTP offset: {}", e),
        }


        let mut bool_loop_listener= true;
        if bool_loop_listener {
        
            let offset:i64; 
            offset = IIDUtility::get_default_global_ntp_offset_in_milliseconds();

            let mut listener = ListenUdpIID::new(
                "0.0.0.0",
                 3615,
                 offset, 1259).unwrap();
            listener.start_listening();
        
        }

        let mut bool_loop_sender= true;
        if bool_loop_sender{
            let mut server_name= "127.0.0.1";
            let mut use_server = false;
            if use_server {
                server_name = "apint.ddns.net";
            }
            let mut sender = SendUdpIID::new(server_name, 3615, true).unwrap();
    
            let mut value: i32 = 0;
            let mut index: i32 = 0;
            loop {
                sender.push_index_integer_date_ntp_in_milliseconds(1, value, 0).unwrap();
                if index%10 == 0 {
                    sender.push_index_integer_date_ntp_in_milliseconds(0,1259, 0);
                }
                index += 1;
                std::thread::sleep(std::time::Duration::from_secs(1));
    
            }   
        }

        while true {
            std::thread::sleep(std::time::Duration::from_secs(1));
            
        }

      
}

pub struct IIDUtility;

impl IIDUtility {
    pub const DEFAULT_NTP_SERVER: &'static str = "be.pool.ntp.org";
    pub const DEFAULT_GLOBAL_NTP_OFFSET_IN_MILLISECONDS: i64 = 0;

    pub fn is_text_ipv4(server_name: &str) -> bool {
        let pattern = Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap();
        pattern.is_match(server_name)
    }

    pub fn get_ipv4(server_name: &str) -> Result<String, Box<dyn Error>> {
        if Self::is_text_ipv4(server_name) {
            return Ok(server_name.to_string());
        }
        let addr = (server_name, 0).to_socket_addrs()?.next().ok_or("Unable to resolve host")?;
        Ok(addr.ip().to_string())
    }

    pub fn get_default_global_ntp_offset_in_milliseconds() -> i64 {
        Self::DEFAULT_GLOBAL_NTP_OFFSET_IN_MILLISECONDS
    }

    pub fn bytes_to_int(bytes: &[u8]) -> Result<i32, Box<dyn Error>> {
        let mut cursor = Cursor::new(bytes);
        Ok(cursor.read_i32::<LittleEndian>()?)
    }

    pub fn bytes_to_index_integer(bytes: &[u8]) -> Result<(i32, i32), Box<dyn Error>> {
        let mut cursor = Cursor::new(bytes);
        let index = cursor.read_i32::<LittleEndian>()?;
        let value = cursor.read_i32::<LittleEndian>()?;
        Ok((index, value))
    }

    pub fn bytes_to_integer_date(bytes: &[u8]) -> Result<(i32, u64), Box<dyn Error>> {
        let mut cursor = Cursor::new(bytes);
        let value = cursor.read_i32::<LittleEndian>()?;
        let date = cursor.read_u64::<LittleEndian>()?;
        Ok((value, date))
    }

    pub fn bytes_to_index_integer_date(bytes: &[u8]) -> Result<(i32, i32, u64), Box<dyn Error>> {
        let mut cursor = Cursor::new(bytes);
        let index = cursor.read_i32::<LittleEndian>()?;
        let value = cursor.read_i32::<LittleEndian>()?;
        let date = cursor.read_u64::<LittleEndian>()?;
        Ok((index, value, date))
    }

    pub fn integer_to_bytes(value: i32) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = vec![];
        buffer.write_i32::<LittleEndian>(value)?;
        Ok(buffer)
    }

    pub fn index_integer_to_bytes(index: i32, value: i32) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = vec![];
        buffer.write_i32::<LittleEndian>(index)?;
        buffer.write_i32::<LittleEndian>(value)?;
        Ok(buffer)
    }

    pub fn integer_date_to_bytes(value: i32, date: u64) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = vec![];
        buffer.write_i32::<LittleEndian>(value)?;
        buffer.write_u64::<LittleEndian>(date)?;
        Ok(buffer)
    }

    pub fn index_integer_date_to_bytes(index: i32, value: i32, date: u64) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer = vec![];
        buffer.write_i32::<LittleEndian>(index)?;
        buffer.write_i32::<LittleEndian>(value)?;
        buffer.write_u64::<LittleEndian>(date)?;
        Ok(buffer)
    }

    pub fn index_integer_now_relay_milliseconds_to_bytes(
        index: i32,
        value: i32,
        delay_in_milliseconds: i64,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let current_time_milliseconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis() as i64;
        let adjusted_time_milliseconds = current_time_milliseconds
            + delay_in_milliseconds
            + Self::DEFAULT_GLOBAL_NTP_OFFSET_IN_MILLISECONDS;
        Self::index_integer_date_to_bytes(index, value, adjusted_time_milliseconds as u64)
    }

    pub fn text_shortcut_to_bytes(text: &str) -> Option<Vec<u8>> {
        let text = text.trim().replace("  ", " ");
        if let Some(integer_text) = text.strip_prefix("i:") {
            let integer = integer_text.parse::<i32>().ok()?;
            Some(Self::integer_to_bytes(integer).ok()?)
        } else if let Some(ii_text) = text.strip_prefix("ii:") {
            let parts: Vec<&str> = ii_text.split(',').collect();
            if parts.len() == 2 {
                let index = parts[0].parse::<i32>().ok()?;
                let value = parts[1].parse::<i32>().ok()?;
                Some(Self::index_integer_to_bytes(index, value).ok()?)
            } else {
                None
            }
        } else if let Some(iid_text) = text.strip_prefix("iid:") {
            let parts: Vec<&str> = iid_text.split(',').collect();
            if parts.len() == 3 {
                let index = parts[0].parse::<i32>().ok()?;
                let value = parts[1].parse::<i32>().ok()?;
                let delay = parts[2].parse::<i64>().ok()?;
                Some(Self::index_integer_now_relay_milliseconds_to_bytes(index, value, delay).ok()?)
            } else {
                None
            }
        } else {
            let tokens: Vec<&str> = text.split_whitespace().collect();
            match tokens.len() {
                1 => {
                    let integer = tokens[0].parse::<i32>().ok()?;
                    Some(Self::integer_to_bytes(integer).ok()?)
                }
                2 => {
                    let index = tokens[0].parse::<i32>().ok()?;
                    let value = tokens[1].parse::<i32>().ok()?;
                    Some(Self::index_integer_to_bytes(index, value).ok()?)
                }
                3 => {
                    let index = tokens[0].parse::<i32>().ok()?;
                    let value = tokens[1].parse::<i32>().ok()?;
                    let delay = tokens[2].parse::<i64>().ok()?;
                    Some(Self::index_integer_now_relay_milliseconds_to_bytes(index, value, delay).ok()?)
                }
                _ => None,
            }
        }
    }


    pub fn get_random_integer(from_value: i32, to_value: i32) -> i32 {
        
        let mut rng = rand::thread_rng();
        rng.gen_range(from_value..=to_value)
    }

    pub fn get_random_integer_100() -> i32 {
        Self::get_random_integer(0, 100)
    }

    pub fn get_random_integer_int_max() -> i32 {
        Self::get_random_integer(i32::MIN + 1, i32::MAX)
    }

    pub fn get_random_integer_int_max_positive() -> i32 {
        Self::get_random_integer(0, i32::MAX)
    }

    pub fn i(integer_value: i32) -> Result<Vec<u8>, Box<dyn Error>> {
        Self::integer_to_bytes(integer_value)
    }

    pub fn ii(index: i32, integer_value: i32) -> Result<Vec<u8>, Box<dyn Error>> {
        Self::index_integer_to_bytes(index, integer_value)
    }

    pub fn iid(index: i32, integer_value: i32, date: u64) -> Result<Vec<u8>, Box<dyn Error>> {
        Self::index_integer_date_to_bytes(index, integer_value, date)
    }

    pub fn iid_ms(index: i32, integer_value: i32, milliseconds: i64) -> Result<Vec<u8>, Box<dyn Error>> {
        Self::index_integer_date_to_bytes(index, integer_value, milliseconds as u64)
    }


}



pub struct NtpOffsetFetcher;

impl NtpOffsetFetcher {

    pub const DEFAULT_NTP_SERVER: &'static str = "be.pool.ntp.org";

    pub fn fetch_ntp_offset_in_milliseconds(ntp_server: &str) -> Result<i64, Box<dyn Error>> {

        // Did not take UTC in consideration:
        // https://github.com/EloiStree/HelloRustBending/blob/main/RustEveryDay/2024_06_28_D19_NTP/hello_ntp/src/main.rs   
        // May check to modify later.
        let client = SntpClient::new();
        let result = client.synchronize(ntp_server).unwrap();
        let clock_offset_abs = result.clock_offset().abs_as_std_duration().unwrap().as_secs_f64();
        let clock_offset = clock_offset_abs * result.clock_offset().signum() as f64;

        println!("Clock offset: {} seconds", clock_offset);
        Ok((clock_offset * 1000.0) as i64)
    }
}



pub struct SendUdpIID {
    ipv4: String,
    port: u16,
    ntp_offset_local_to_server_in_milliseconds: i64,
    sock: UdpSocket,
}

impl SendUdpIID {
    pub fn new(ipv4: &str, port: u16, use_ntp: bool) -> Result<Self, Box<dyn Error>> {
        let ipv4 = IIDUtility::get_ipv4(ipv4)?;
        let sock = UdpSocket::bind("0.0.0.0:0")?;
        let mut instance = SendUdpIID {
            ipv4,
            port,
            ntp_offset_local_to_server_in_milliseconds: 0,
            sock,
        };
        if use_ntp {
            instance.fetch_ntp_offset(NtpOffsetFetcher::DEFAULT_NTP_SERVER)?;
            //instance.ntp_offset_local_to_server_in_milliseconds= NtpOffsetFetcher::fetch_ntp_offset_in_milliseconds(NtpOffsetFetcher::DEFAULT_NTP_SERVER)?;
        }
        Ok(instance)
    }

    pub fn get_ntp_offset(&self) -> i64 {
        self.ntp_offset_local_to_server_in_milliseconds
    }

    pub fn push_integer_as_shortcut(&self, text: &str) -> Result<(), Box<dyn Error>> {
        if let Some(bytes) = IIDUtility::text_shortcut_to_bytes(text) {
            self.sock.send_to(&bytes, (self.ipv4.as_str(), self.port))?;
        }
        Ok(())
    }

    pub fn push_bytes(&self, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        self.sock.send_to(bytes, (self.ipv4.as_str(), self.port))?;
        println!("Push Bytes: {} {} {:?}", self.ipv4, self.port, bytes);
        Ok(())
    }

    pub fn push_text(&self, text: &str) -> Result<(), Box<dyn Error>> {
        self.push_bytes(text.as_bytes())
    }

    pub fn push_integer(&self, value: i32) -> Result<(), Box<dyn Error>> {
        self.push_bytes(&IIDUtility::integer_to_bytes(value)?)
    }

    pub fn push_index_integer(&self, index: i32, value: i32) -> Result<(), Box<dyn Error>> {
        self.push_bytes(&IIDUtility::index_integer_to_bytes(index, value)?)
    }

    pub fn push_index_integer_date(&self, index: i32, value: i32, date: u64) -> Result<(), Box<dyn Error>> {
        self.push_bytes(&IIDUtility::index_integer_date_to_bytes(index, value, date)?)
    }

    pub fn push_random_integer(&self, index: i32, from_value: i32, to_value: i32) -> Result<(), Box<dyn Error>> {
        let value = IIDUtility::get_random_integer(from_value, to_value);
        self.push_index_integer(index, value)
    }

    pub fn push_random_integer_100(&self, index: i32) -> Result<(), Box<dyn Error>> {
        self.push_random_integer(index, 0, 100)
    }

    pub fn push_random_integer_int_max(&self, index: i32) -> Result<(), Box<dyn Error>> {
        self.push_random_integer(index, i32::MIN + 1, i32::MAX)
    }

    pub fn fetch_ntp_offset(&mut self, ntp_server: &str) -> Result<(), Box<dyn Error>> {
        self.ntp_offset_local_to_server_in_milliseconds = NtpOffsetFetcher::fetch_ntp_offset_in_milliseconds(ntp_server)?;
        println!("NTP Offset: {}", self.ntp_offset_local_to_server_in_milliseconds);
        Ok(())
    }

    pub fn set_ntp_offset_tick(&mut self, ntp_offset_local_to_server: i64) {
        self.ntp_offset_local_to_server_in_milliseconds = ntp_offset_local_to_server;
    }

    pub fn push_index_integer_date_local_now(&self, index: i32, value: i32) -> Result<(), Box<dyn Error>> {
        let date = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as u64;
        self.push_index_integer_date(index, value, date)
    }

    pub fn push_index_integer_date_ntp_now(&self, index: i32, value: i32) -> Result<(), Box<dyn Error>> {
        let date = (SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64 + self.ntp_offset_local_to_server_in_milliseconds) as u64;
        self.push_index_integer_date(index, value, date)
    }

    pub fn push_index_integer_date_ntp_in_milliseconds(&self, index: i32, value: i32, milliseconds: i64) -> Result<(), Box<dyn Error>> {
        let date = (SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64 + self.ntp_offset_local_to_server_in_milliseconds + milliseconds) as u64;
        self.push_index_integer_date(index, value, date)
    }

    pub fn push_index_integer_date_ntp_in_seconds(&self, index: i32, value: i32, seconds: i64) -> Result<(), Box<dyn Error>> {
        self.push_index_integer_date_ntp_in_milliseconds(index, value, seconds * 1000)
    }
}




// fn main() -> Result<(), Error> {
//     println!("Hello, world!");
//     println!("Start receiving random numbers from the client... port 7000");
//     let socket = UdpSocket::bind("127.0.0.1:7000")?;
//     loop {
//         let mut buf = [0; 4]; // Buffer to hold the received bytes
//         let (amt, src) = socket.recv_from(&mut buf)?;

//         let number = i32::from_be_bytes(buf); // Convert the bytes back to an integer

//         println!("Received: {} from {}", number, src);
//     }
//     Ok(())
// }


//




pub struct ListenUdpIID {
    ipv4: String,
    port: u16,

    ntp_offset_in_milliseconds: i64,
    manual_adjustment_source_to_local_ntp_offset_in_milliseconds: i64,
    integer_to_sync_ntp: i32,

    on_receive_integer: Option<Arc<Mutex<dyn Fn(i32) + Send>>>,
    on_receive_index_integer: Option<Arc<Mutex<dyn Fn(i32, i32) + Send>>>,
    on_receive_index_integer_date: Option<Arc<Mutex<dyn Fn(i32, i32, u64) + Send>>>,
    on_received_integer_date: Option<Arc<Mutex<dyn Fn(i32, u64) + Send>>>,
}
impl ListenUdpIID {
    pub fn new(ipv4: &str, port: u16, ntp_offset_in_milliseconds: i64, integer_to_sync_ntp: i32) -> Result<Self, Box<dyn Error>> {
        let ipv4 = IIDUtility::get_ipv4(ipv4)?;
        let mut instance = ListenUdpIID {
            ipv4,
            port,
            ntp_offset_in_milliseconds,
            manual_adjustment_source_to_local_ntp_offset_in_milliseconds: 0,
            integer_to_sync_ntp,
            on_receive_integer: None,
            on_receive_index_integer: None,
            on_receive_index_integer_date: None,
            on_received_integer_date: None,
        };
        instance.set_receive_as_debug();
        Ok(instance)
    }

    pub fn set_receive_as_debug(&mut self) {
        self.on_receive_integer = Some(Arc::new(Mutex::new(|value| {
            println!("Received Integer: {}", value);
        })));
        self.on_receive_index_integer = Some(Arc::new(Mutex::new(|index, value| {
            println!("Received Index Integer: {} {}", index, value);
        })));
        self.on_receive_index_integer_date = Some(Arc::new(Mutex::new(|index, value, date| {
            println!("Received Index Integer Date: {} {} {}", index, value, date);
        })));
        self.on_received_integer_date = Some(Arc::new(Mutex::new(|value, date| {
            println!("Received Integer Date: {} {}", value, date);
        })));
    }

    pub fn debug_received_integer(&self, value: i32) {
        println!("Received Integer: {}", value);
    }

    pub fn debug_received_index_integer(&self, index: i32, value: i32) {
        println!("Received Index Integer: {} {}", index, value);
    }

    pub fn debug_received_integer_date(&self, value: i32, date: u64) {
        let time = Self::get_ntp_time_in_milliseconds(self.ntp_offset_in_milliseconds);
        println!("Received Integer Date: {} {} vs {} dif {}", value, date, time, time as i64 - date as i64);
    }

    pub fn debug_received_index_integer_date(&self, index: i32, value: i32, date: u64) {
        let time = Self::get_ntp_time_in_milliseconds(self.ntp_offset_in_milliseconds);
        println!("Received Index Integer Date: {} {} {} vs {} dif {}", index, value, date, time, time as i64 - date as i64);
    }

    pub fn start_listening(&self) {
        let ipv4: String = self.ipv4.clone();
        let port: u16 = self.port;

        let sock: UdpSocket = UdpSocket::bind((ipv4.as_str(), port)).expect("Failed to bind socket");

        let ntp_offset_in_milliseconds = self.ntp_offset_in_milliseconds;
        let manual_adjustment_source_to_local_ntp_offset_in_milliseconds = Arc::new(Mutex::new(self.manual_adjustment_source_to_local_ntp_offset_in_milliseconds));
        let integer_to_sync_ntp = self.integer_to_sync_ntp;
        let on_receive_integer = self.on_receive_integer.clone();
        let on_receive_index_integer = self.on_receive_index_integer.clone();
        let on_receive_index_integer_date = self.on_receive_index_integer_date.clone();
        let on_received_integer_date = self.on_received_integer_date.clone();

        thread::spawn(move || {
            let mut buf = [0; 1024];
            loop {
                match sock.recv_from(&mut buf) {
                    Ok((size, _src)) => {
                        if size == 4 {
                            if let Ok(value) = IIDUtility::bytes_to_int(&buf[..size]) {
                                if let Some(callback) = &on_receive_integer {
                                    let callback = callback.lock().unwrap();
                                    callback(value);
                                }
                            }
                        } else if size == 8 {
                            if let Ok((index, value)) = IIDUtility::bytes_to_index_integer(&buf[..size]) {
                                if let Some(callback) = &on_receive_index_integer {
                                    let callback = callback.lock().unwrap();
                                    callback(index, value);
                                }
                            }
                        } else if size == 12 {
                            if let Ok((value, date)) = IIDUtility::bytes_to_integer_date(&buf[..size]) {
                                if value == integer_to_sync_ntp {
                                    let local_time = Self::get_ntp_time_in_milliseconds(ntp_offset_in_milliseconds);
                                    let mut adjustment = manual_adjustment_source_to_local_ntp_offset_in_milliseconds.lock().unwrap();
                                    *adjustment = local_time as i64 - date as i64;
                                }
                                if let Some(callback) = &on_received_integer_date {
                                    let callback = callback.lock().unwrap();
                                    callback(value, date);
                                }
                            }
                        } else if size == 16 {
                            if let Ok((index, value, date)) = IIDUtility::bytes_to_index_integer_date(&buf[..size]) {
                                if value == integer_to_sync_ntp {
                                    let local_time = Self::get_ntp_time_in_milliseconds(ntp_offset_in_milliseconds);
                                    let mut adjustment = manual_adjustment_source_to_local_ntp_offset_in_milliseconds.lock().unwrap();
                                    *adjustment = local_time as i64 - date as i64;
                                }
                                if let Some(callback) = &on_receive_index_integer_date {
                                    let callback = callback.lock().unwrap();
                                    callback(index, value, date);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving data: {}", e);
                        break;
                    }
                }
            }
        });
    }

    fn get_ntp_time_in_milliseconds(ntp_offset_in_milliseconds: i64) -> u64 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        (now + ntp_offset_in_milliseconds) as u64
    }
}
