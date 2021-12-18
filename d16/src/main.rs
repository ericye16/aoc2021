use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: u8,
    packet_type: u8,
    payload: Payload,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Payload {
    Literal(Vec<u8>),
    Subpackets(Vec<Packet>),
}

impl Payload {
    fn to_literal(&self) -> Option<&Vec<u8>> {
        match &self {
            &Self::Literal(o) => Some(o),
            _ => None,
        }
    }

    fn to_subpackets(&self) -> Option<&Vec<Packet>> {
        match &self {
            &Self::Subpackets(o) => Some(o),
            _ => None,
        }
    }
}

fn preprocess_packet(input: &str) -> BitVec<Msb0, u8> {
    hex::decode(input).unwrap().view_bits::<Msb0>().to_owned()
}

fn parse_subpackets(bit_slice: &BitSlice<Msb0, u8>) -> (Vec<Packet>, usize) {
    match bit_slice[0] {
        false => {
            // Length type 0 -- then the next 15 bits are a number that represents
            // the total length in bits of the sub-packets contained by this packet
            let length_subpackets = bit_slice[1..(1 + 15)].load_be::<usize>();
            let mut i = 16;
            let mut v = vec![];
            while i < (16 + length_subpackets) {
                let (next_packet, n) = parse_packet(&bit_slice[i..]);
                v.push(next_packet);
                i += n;
            }
            (v, i)
        }
        true => {
            // the length type ID is 1, then the next 11 bits are a number that
            // represents the number of sub-packets immediately contained by this packet.
            let num_subpackets = bit_slice[1..(1 + 11)].load_be::<usize>();
            let mut i = 12;
            let mut v = vec![];
            for _ in 0..num_subpackets {
                let (next_packet, n) = parse_packet(&bit_slice[i..]);
                v.push(next_packet);
                i += n;
            }
            (v, i)
        }
    }
}

fn parse_packet(bit_slice: &BitSlice<Msb0, u8>) -> (Packet, usize) {
    let version = bit_slice[0..3].load_be::<u8>();
    let packet_type = bit_slice[3..6].load_be::<u8>();
    let (payload, next_bit) = match packet_type {
        4 => {
            // literal
            let mut has_more = true;
            let mut i = 6;
            let mut literal_vec = vec![];
            while has_more {
                has_more = bit_slice[i];
                i += 1;
                literal_vec.push(bit_slice[i..(i + 4)].load_be::<u8>());
                i += 4;
            }
            (Payload::Literal(literal_vec), i)
        }
        _ => {
            // operator
            let (subpackets, next_bit) = parse_subpackets(&bit_slice[6..]);
            (Payload::Subpackets(subpackets), next_bit + 6)
        }
    };
    (
        Packet {
            version,
            packet_type,
            payload,
        },
        next_bit,
    )
}

fn sum_version(packet: &Packet) -> u32 {
    let mut sum = packet.version as u32;
    if let Payload::Subpackets(subpackets) = &packet.payload {
        sum += subpackets.iter().map(sum_version).sum::<u32>()
    }
    sum
}

fn p1(input: &str) -> u32 {
    let bits = preprocess_packet(input);
    let (packet, _) = parse_packet(&bits);
    sum_version(&packet)
}

fn calculate(packet: &Packet) -> i64 {
    match packet.packet_type {
        4 => {
            // Literal
            let mut val = 0;
            for nibble in packet.payload.to_literal().unwrap() {
                val <<= 4;
                val += *nibble as i64;
            }
            val
        }
        0 => {
            // sum
            packet
                .payload
                .to_subpackets()
                .unwrap()
                .iter()
                .map(calculate)
                .sum()
        }
        1 => {
            // product
            packet
                .payload
                .to_subpackets()
                .unwrap()
                .iter()
                .map(calculate)
                .product()
        }
        2 => {
            // min
            packet
                .payload
                .to_subpackets()
                .unwrap()
                .iter()
                .map(calculate)
                .min()
                .unwrap()
        }
        3 => {
            // max
            packet
                .payload
                .to_subpackets()
                .unwrap()
                .iter()
                .map(calculate)
                .max()
                .unwrap()
        }
        5 => {
            // gt
            assert_eq!(packet.payload.to_subpackets().unwrap().len(), 2);
            let subpackets = packet.payload.to_subpackets().unwrap();
            if calculate(&subpackets[0]) > calculate(&subpackets[1]) {
                1
            } else {
                0
            }
        }
        6 => {
            // lt
            assert_eq!(packet.payload.to_subpackets().unwrap().len(), 2);
            let subpackets = packet.payload.to_subpackets().unwrap();
            if calculate(&subpackets[0]) < calculate(&subpackets[1]) {
                1
            } else {
                0
            }
        }
        7 => {
            // eq
            assert_eq!(packet.payload.to_subpackets().unwrap().len(), 2);
            let subpackets = packet.payload.to_subpackets().unwrap();
            if calculate(&subpackets[0]) == calculate(&subpackets[1]) {
                1
            } else {
                0
            }
        }
        _ => {
            panic!()
        }
    }
}

fn p2(input: &str) -> i64 {
    let bits = preprocess_packet(input);
    let (packet, _) = parse_packet(&bits);
    calculate(&packet)
}

fn main() {
    let input = common::read_file("d16.txt");
    println!("P1: {}", p1(input.trim()));
    println!("P2: {}", p2(input.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "8A004A801A8002F478";
    const I_2: &str = "620080001611562C8802118E34";
    const I_3: &str = "C0015000016115A2E0802F182340";
    const I_4: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn test_parse_packet() {
        let i1 = preprocess_packet("D2FE28");
        let o1 = Packet {
            version: 6,
            packet_type: 4,
            payload: Payload::Literal(vec![0b0111, 0b1110, 0b101]),
        };
        let packet = parse_packet(&i1);
        assert_eq!(&packet, &(o1, 21));
        assert_eq!(calculate(&packet.0), 2021);
    }

    #[test]
    fn test_parse_packet2() {
        let input = preprocess_packet("38006F45291200");
        let (output, l) = parse_packet(&input);
        assert_eq!(l, 49);
        assert_eq!(output.version, 1);
        assert_eq!(output.packet_type, 6);
        let subpackets = output.payload.to_subpackets().unwrap();
        assert_eq!(subpackets.len(), 2);
        assert_eq!(*subpackets[0].payload.to_literal().unwrap(), vec![10]);
        assert_eq!(*subpackets[1].payload.to_literal().unwrap(), vec![1, 4]);
    }

    #[test]
    fn test_parse_packet3() {
        let input = preprocess_packet("EE00D40C823060");
        let (output, l) = parse_packet(&input);
        assert_eq!(l, 51);
        assert_eq!(output.version, 7);
        assert_eq!(output.packet_type, 3);
        let subpackets = output.payload.to_subpackets().unwrap();
        assert_eq!(subpackets.len(), 3);
        assert_eq!(*subpackets[0].payload.to_literal().unwrap(), vec![1]);
        assert_eq!(*subpackets[1].payload.to_literal().unwrap(), vec![2]);
        assert_eq!(*subpackets[2].payload.to_literal().unwrap(), vec![3]);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(INPUT), 16);
        assert_eq!(p1(I_2), 12);
        assert_eq!(p1(I_3), 23);
        assert_eq!(p1(I_4), 31);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("C200B40A82"), 3);
        assert_eq!(p2("04005AC33890"), 54);
        assert_eq!(p2("880086C3E88112"), 7);
        assert_eq!(p2("CE00C43D881120"), 9);
        assert_eq!(p2("D8005AC2A8F0"), 1);
        assert_eq!(p2("F600BC2D8F"), 0);
        assert_eq!(p2("9C005AC2F8F0"), 0);
        assert_eq!(p2("9C0141080250320F1802104A08"), 1);
    }
}
