use std::time::{ Instant };
use std::net::{ SocketAddr, TcpStream };


fn main() {
    let ip = match std::env::args().nth(1) {
        Some(ip) => ip,
        __ => {
            colour::e_red!("Erro: ");
            print!("Você esqueceu de colocar o ip e a porta.\n");

            return ();
        }
    };

    let server: SocketAddr = match ip.parse() {
        Ok(server) => server,
        Err(_) => {
            colour::e_red!("Erro: ");
            print!("IP Invalido. Utilize: tcpping ip:porta.\n");

            return ()
        }
    };


    colour::prnt!("\nConectando com ");
    colour::e_yellow!("{}", server.ip().to_string());
    colour::prnt!(" em ");
    colour::e_yellow!("TCP {}\n\n", server.port().to_string());


    loop {
        let time = Instant::now();

        let socket = TcpStream::connect_timeout(&server, std::time::Duration::from_millis(5000));


        if socket.is_ok() {
            colour::prnt!("Conectado com ");
            colour::e_green!("{}", server.ip());
            colour::prnt!(": ping=");
            colour::e_green!("{}ms", time.elapsed().as_millis());
            colour::prnt!(" protocolo=");
            colour::e_green!("TCP");
            colour::prnt!(" porta=");
            colour::e_green!("{}\n", server.port());

            socket.unwrap().shutdown(std::net::Shutdown::Both).unwrap_or_default();
        } else {
            colour::e_red!("Não foi possivel estabelecer a conexão com o servidor\n")
        }


        std::thread::sleep(std::time::Duration::from_millis(1000))
    }
}
