#[cfg(target_family = "wasm")]
/// Polyfill that provides mio like API;
/// mio does not support WASI networking yet;
pub mod mio {
    use std::io;
    use std::time::Duration;
    use std::os::fd::AsRawFd;

    pub struct Poll {
        socket_raw_fd: Option<i32>,
        token: Option<Token>,
    }

    impl Poll {
        pub fn new() -> Result<Self, ()> {
            Ok(Self {
                socket_raw_fd: None,
                token: None,
            })
        }

        pub fn register(&mut self, socket: &mut net::UdpSocket, token: Token, interest: Interest) -> Result<(),()> {
            assert_eq!(self.socket_raw_fd, None);
            assert_eq!(self.token, None);
            assert_eq!(interest, Interest::READABLE);
            self.socket_raw_fd = Some(socket.inner.as_raw_fd());
            self.token = Some(token);
            Ok(())
        }

        pub fn poll(&mut self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()> {
            events.inner.clear();
            let mut fds = [libc::pollfd {
                fd: self.socket_raw_fd.unwrap(),
                events: libc::POLLIN,
                revents: 0,
            }];
            let poll_ret = unsafe {
                libc::poll(fds.as_mut_ptr(), fds.len() as u32, match timeout {
                    Some(timeout) => timeout.as_millis() as i32,
                    None => -1 as i32, // Specifying a negative value in timeout means an infinite timeout
                })
            };
            let is_timeout = poll_ret == 0;
            let is_error = poll_ret < 0;
            assert!(!is_error);
            if !is_timeout {
                events.inner.push(Event{
                    token: self.token.unwrap(),
                })
            }
            Ok(())
        }

        pub fn registry(&mut self) -> &mut Self {
            self
        }
    }

    pub struct Events {
        inner: Vec<Event>
    }

    impl Events {
        pub fn with_capacity(capacity: usize) -> Events {
            Self {
                inner: Vec::with_capacity(capacity)
            }
        }
        pub fn is_empty(&self) -> bool {
            self.inner.is_empty()
        }
    }

    impl<'a> IntoIterator for &'a Events {
        type Item = Event;
        type IntoIter = std::iter::Copied<std::slice::Iter<'a, Event>>;
        fn into_iter(self) -> Self::IntoIter {
            self.inner.iter().copied()
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Event{
        pub token: Token,
    }

    impl Event {
        pub fn token(&self) -> Token {
            self.token
        }
    }

    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    pub struct Interest(u8);

    impl Interest {
        pub const READABLE: Interest = Interest(1);
    }

    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    pub struct Token(pub usize);

    pub mod net {
        pub struct UdpSocket {
            pub inner: std::net::UdpSocket
        }

        impl UdpSocket {
            pub fn bind(addr: std::net::SocketAddr) -> Result<UdpSocket, std::io::Error> {
                let inner = std::net::UdpSocket::bind(addr)?;
                inner.set_nonblocking(true).unwrap();
                Ok(Self{
                    inner,
                })
            }

            pub fn send_to(&self, buf: &[u8], addr: std::net::SocketAddr) -> Result<usize, std::io::Error> {
                self.inner.send_to(buf, addr)
            }

            pub fn local_addr(&self) -> Result<std::net::SocketAddr, std::io::Error> {
                self.inner.local_addr()
            }

            pub fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, std::net::SocketAddr), std::io::Error> {
                self.inner.recv_from(buf)
            }
        }
    }
}
