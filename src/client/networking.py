import socket
import pickle, json

BROADCAST_PORT = 54432


def get_server_ip(broadcast_port: int):
    sock: socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
    sock.bind(("0.0.0.0", broadcast_port))
    
    print("Getting server ip...")

    data, addr = sock.recvfrom(1024)

    while not data.decode().startswith("PKR BROADCAST"):
        data, addr = sock.recvfrom(1024)

    print(f"Got broadcast from {addr}")
    print(f"Got data: {data}")
    print(f"Returning sent ip: {data.decode()[14:]}")
    print("Closing getting ip socket function")
    sock.close()
    return data.decode()[14:]

def handshake(addr: socket.socket) -> bool:
    if addr.recv(1024) == b"PKER GAME":
        addr.send(b"YES")
        if addr.recv(1024) == b"CONFIRM":
            # send data to server
            # player name TODO: add way to add name
            # game net version
            # more data idk
            
            # server will put name
            data = {"GAME_VERSION": "0.1"}
            addr.send(json.dumps(data).encode())
            
            return True
    else:
        return False

def connect() -> socket.socket:
    sock: socket.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    address: str = get_server_ip(BROADCAST_PORT)
    port: int = 50433 #TODO: get from configs

    print(f"Connecting to server at ip: {address}")

    sock.connect((address, port))
    
    return sock


def get_cards(server: socket.socket) -> tuple[list[int, int], list[int, int]]:
    return pickle.loads(server.recv(1024))