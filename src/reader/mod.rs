use crate::solver::Node;
use std::{error, str};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStr;

#[derive(Debug)]
pub struct TspData {
    pub nodes: Vec<Node>,
}

impl TspData {
    pub fn new(nodes: Vec<Node>) -> TspData {
        TspData { nodes }
    }
}

impl str::FromStr for TspData {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TspData::new(
            s.split('\n').enumerate()
                .map(|(i, s)| parse_node_coord_section_line(i+1,s).unwrap())
                .collect(),
        ))
    }
}

pub fn read(path: &str) -> Result<TspData, Box<dyn error::Error + 'static>> {
    let data: TspData = std::fs::read_to_string(path)?.parse()?;
    Ok(data)
}

fn parse_node_coord_section_line(index: usize, line: &str) -> Result<Node, &'static str> {
    let mut parsed_line: Vec<&str> = line
        .split_whitespace()
        .collect();

    return match parsed_line.len() {
        2 => {
            let y = Decimal::from_str(parsed_line.pop().unwrap()).unwrap();
            let x = Decimal::from_str(parsed_line.pop().unwrap()).unwrap();
            Ok(Node::new(index as u32, x, y))
        }
        _ => Err("line too short"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn dj38() {
        let data = read(&"instances/dj38.tsp").unwrap();
        let expected_node_coords = vec![
            Node::new(1,Decimal::from_str("11003.611100").unwrap(), Decimal::from_str("42102.500000").unwrap()),
            Node::new(2,Decimal::from_str("11108.611100").unwrap(), Decimal::from_str("42373.888900").unwrap()),
            Node::new(3, Decimal::from_str("11133.333300").unwrap(), Decimal::from_str("42885.833300").unwrap()),
            Node::new(4, Decimal::from_str("11155.833300").unwrap(), Decimal::from_str("42712.500000").unwrap()),
            Node::new(5, Decimal::from_str("11183.333300").unwrap(), Decimal::from_str("42933.333300").unwrap()),
            Node::new(6, Decimal::from_str("11297.500000").unwrap(), Decimal::from_str("42853.333300").unwrap()),
            Node::new(7,Decimal::from_str("11310.277800").unwrap(), Decimal::from_str("42929.444400").unwrap()),
            Node::new(8,Decimal::from_str("11416.666700").unwrap(), Decimal::from_str("42983.333300").unwrap()),
            Node::new(9,Decimal::from_str("11423.888900").unwrap(), Decimal::from_str("43000.277800").unwrap()),
            Node::new(10,Decimal::from_str("11438.333300").unwrap(), Decimal::from_str("42057.222200").unwrap()),
            Node::new(11,Decimal::from_str("11461.111100").unwrap(), Decimal::from_str("43252.777800").unwrap()),
            Node::new(12,Decimal::from_str("11485.555600").unwrap(), Decimal::from_str("43187.222200").unwrap()),
            Node::new(13,Decimal::from_str("11503.055600").unwrap(), Decimal::from_str("42855.277800").unwrap()),
            Node::new(14,Decimal::from_str("11511.388900").unwrap(), Decimal::from_str("42106.388900").unwrap()),
            Node::new(15,Decimal::from_str("11522.222200").unwrap(), Decimal::from_str("42841.944400").unwrap()),
            Node::new(16,Decimal::from_str("11569.444400").unwrap(), Decimal::from_str("43136.666700").unwrap()),
            Node::new(17,Decimal::from_str("11583.333300").unwrap(), Decimal::from_str("43150.000000").unwrap()),
            Node::new(18,Decimal::from_str("11595.000000").unwrap(), Decimal::from_str("43148.055600").unwrap()),
            Node::new(19,Decimal::from_str("11600.000000").unwrap(), Decimal::from_str("43150.000000").unwrap()),
            Node::new(20,Decimal::from_str("11690.555600").unwrap(), Decimal::from_str("42686.666700").unwrap()),
            Node::new(21,Decimal::from_str("11715.833300").unwrap(), Decimal::from_str("41836.111100").unwrap()),
            Node::new(22,Decimal::from_str("11751.111100").unwrap(), Decimal::from_str("42814.444400").unwrap()),
            Node::new(23,Decimal::from_str("11770.277800").unwrap(), Decimal::from_str("42651.944400").unwrap()),
            Node::new(24,Decimal::from_str("11785.277800").unwrap(), Decimal::from_str("42884.444400").unwrap()),
            Node::new(25,Decimal::from_str("11822.777800").unwrap(), Decimal::from_str("42673.611100").unwrap()),
            Node::new(26,Decimal::from_str("11846.944400").unwrap(), Decimal::from_str("42660.555600").unwrap()),
            Node::new(27,Decimal::from_str("11963.055600").unwrap(), Decimal::from_str("43290.555600").unwrap()),
            Node::new(28,Decimal::from_str("11973.055600").unwrap(), Decimal::from_str("43026.111100").unwrap()),
            Node::new(29,Decimal::from_str("12058.333300").unwrap(), Decimal::from_str("42195.555600").unwrap()),
            Node::new(30,Decimal::from_str("12149.444400").unwrap(), Decimal::from_str("42477.500000").unwrap()),
            Node::new(31,Decimal::from_str("12286.944400").unwrap(), Decimal::from_str("43355.555600").unwrap()),
            Node::new(32,Decimal::from_str("12300.000000").unwrap(), Decimal::from_str("42433.333300").unwrap()),
            Node::new(33,Decimal::from_str("12355.833300").unwrap(), Decimal::from_str("43156.388900").unwrap()),
            Node::new(34,Decimal::from_str("12363.333300").unwrap(), Decimal::from_str("43189.166700").unwrap()),
            Node::new(35,Decimal::from_str("12372.777800").unwrap(), Decimal::from_str("42711.388900").unwrap()),
            Node::new(36,Decimal::from_str("12386.666700").unwrap(), Decimal::from_str("43334.722200").unwrap()),
            Node::new(37,Decimal::from_str("12421.666700").unwrap(), Decimal::from_str("42895.555600").unwrap()),
            Node::new(38,Decimal::from_str("12645.000000").unwrap(), Decimal::from_str("42973.333300").unwrap()),
        ];
        assert_eq!(data.nodes, expected_node_coords);
    }
}
