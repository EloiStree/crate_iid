IID & 🍺.io: https://buymeacoffee.com/apintio - https://github.com/EloiStree/IID - https://github.com/EloiStree/apint.io

--------------------------------------


**Rust Package for IID**  
This code defines what an IID is in Rust.  
It also exists in Python, C# (NuGet/Visual Studio and OpenUPM/Unity), and Rust.  

IID stands for **Index Integer Date**.  
The format is: `i32, i32, u64`.  
You can learn more about it here: [GitHub - EloiStree/iid](https://github.com/EloiStree/iid).  

The aim of IID is to enable networked shared servers to perform remote-controlled actions and support integer-based massive multiplayer games.  

I am not a Rust developer, and I know my code is not great at the moment.  
I am creating this package because I plan to continue learning and improving over my lifetime.  



**Listen to IID:**  
``` rust 
fn main() {
    println!("Hello, UDP IID LISTENER!");
    
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
}

```


**Push some IID:**  
``` rust 
fn main() {
    println!("Hello, UDP IID PUSHER!");
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
}

```



```
/*
 * ----------------------------------------------------------------------------
 * "PIZZA LICENSE":
 * https://github.com/EloiStree wrote this file.
 * As long as you retain this notice, you
 * can do whatever you want with this code.
 * If you think my code saved you time,
 * consider sending me a 🍺 or a 🍕 at:
 *  - https://buymeacoffee.com/apintio
 * 
 * You can also support my work by building your own DIY input device
 * using these Amazon links:
 * - https://github.com/EloiStree/HelloInput
 *
 * May the code be with you.
 *
 * Updated version: https://github.com/EloiStree/License
 * ----------------------------------------------------------------------------
 */
```