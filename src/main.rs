use std::fmt::Formatter;
use std::io::Read;
use serde::{Serialize, Deserialize};
use serde_json;
use std::net::{SocketAddr, TcpListener};

/*
----------               --------------
server:80 | <-- HTTP -> | client:26672 |
----------               --------------
     ^
     |
-------------
client:26673 |
-------------

TCP: Mode Connecté (lettre avec accusé de réception)
UDP: Mode paquet (lettre simple)
*/


#[derive(Debug, Serialize, Deserialize)]
struct Welcome {
    version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Hello,
    Welcome(Welcome),
    // Welcome { version: u8 }
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Message {
//     #[serde(rename = "Welcome")]
//     welcome: Welcome
// }


// impl std::fmt::Debug for Hello {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

fn nonzero_number(n: i32) -> Option<i32> {
    if n == 0 {
        None
    } else {
        Some(n)
    }
}


fn main() {
    // let h = Message { welcome : Welcome { version: 2 }};
    // let h = Message::Welcome { version: 2 };
    let h = Message::Welcome(Welcome { version: 2 });
    // let h = Message::Hello;

    let serialized = serde_json::to_string(&h);
    match serialized {
        Ok(str) => {
            println!("ok:{str}")
        }
        Err(err) => {
            println!("{err}")
        }
    }

    let address = SocketAddr::from(([127, 0, 0, 1], 7878));
    let listener = TcpListener::bind(address);
    
    // let listener = listener.unwrap();
    let listener = match listener {
        Ok(l) => l,
        Err(err) => panic!("Cannot listen on port : {err:?}")
    };
    
    for message in listener.incoming() {
        println!("message={message:?}");
        let mut message = message.unwrap();
        let mut v = Vec::<u8>::new();
        message.read_to_end(&mut v);
        let str = String::from_utf8_lossy(&v);
        println!("{str:?}");
        
        if str == "Hello" {
            
            
        }
        
        
    }
}
