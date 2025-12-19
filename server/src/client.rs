use std::net::SocketAddr;
use network::PeerId;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum State {
    Connecting, // В процессе подключения
    Connected,  // Успешно подключен
}

#[derive(Debug)]
pub struct Client {
    id: i32,         // ID игрока в SA:MP
    state: State,    // Текущее состояние
    addr: SocketAddr, // Адрес клиента
    peer: PeerId,    // ID пира в сети
}

impl Client {
    // Создание нового клиента
    pub fn new(id: i32, peer: PeerId, addr: SocketAddr) -> Client {
        Client {
            id,
            addr,
            peer,
            state: State::Connecting, // По умолчанию в состоянии подключения
        }
    }

    // Получение ID игрока
    pub fn id(&self) -> i32 {
        self.id
    }

    // Установка состояния
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    // Получение адреса клиента
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    // Получение ID пира
    pub fn peer(&self) -> PeerId {
        self.peer
    }

    // Проверка состояния подключения (оставлено, но помечено как allow(dead_code))
    #[allow(dead_code)]
    pub fn is_connected(&self) -> bool {
        self.state == State::Connected
    }
}