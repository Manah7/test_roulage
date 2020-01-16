use heapless::consts::U512;
use librobot::transmission::navigation::{NavigationCommand, NavigationFrame};
use librobot::transmission::Jsonizable;
use std::net::UdpSocket; // type level integer used to specify capacity
use std::{thread, time};


//Interpretation du mouvement
fn normalize(cmd: &NavigationCommand, data: u16) -> u16 {
    match cmd {
        NavigationCommand::GoForward => data * 100,
        NavigationCommand::GoBackward => data * 100,
        NavigationCommand::TurnRelative => (data as f32 * (314.15 / 180.)) as u16,
        NavigationCommand::TurnAbsolute => (data as f32 * (314.15 / 180.)) as u16,
        _ => unreachable!(),
    }
}

//Communication
fn action(cmd: &NavigationCommand, data: u16) {
    let mut frame = NavigationFrame::default();
        //Reglage de la frame
        frame.counter = 1;
        frame.asserv_lin = true;
        frame.asserv_ang = true;
        frame.command = cmd;
        frame.args_cmd1 = normalize(&cmd, data);
        frame.args_cmd2 = 0;

        println!("Data : {}", frame.args_cmd1);
        println!("Sending...");
        socket
            .send(
                frame
                    .to_string::<U512>()
                    .expect("Failed JSON ser")
                    .as_bytes(),
            )
            .unwrap();
        println!("Done !");

}

fn main() {
    println!("Programme de test du roulage");
    let mut cpt = 1;
    let socket = UdpSocket::bind("0.0.0.0:5001").expect("couldn't bind to address");
    socket.connect("192.168.2.1:51").unwrap();

    //Pcq si on met pas dans une variable, ça plante...
    let hd_mil = time::Duration::from_millis(100);

    loop {
        action(NavigationCommand::GoForward, 10);
        thread::sleep(hd_mil);
        action(NavigationCommand::GoBackward, 10);
        thread::sleep(hd_mil);
        action(NavigationCommand::TurnAbsolute, 5000);
        thread::sleep(hd_mil);
        action(NavigationCommand::TurnAbsolute, 0);
        thread::sleep(hd_mil);
    }
}
