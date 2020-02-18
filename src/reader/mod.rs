#[derive(Debug)]
pub struct TspData {
    pub node_coords: Vec<NodeCoord>,
}

#[derive(Debug, PartialEq)]
pub struct NodeCoord {
    pub index: i32,
    pub x: f32,
    pub y: f32,
}

impl NodeCoord {
    fn new(index: i32, x: f32, y: f32) -> NodeCoord {
        NodeCoord { index, x, y }
    }
}

impl TspData {
    pub fn new(node_coords: Vec<NodeCoord>) -> TspData {
        TspData { node_coords }
    }

    pub fn generate_distance_matrix(&self) -> Vec<Vec<f32>> {
        let mut distance_matrix: Vec<Vec<f32>> = Vec::new();
        for coord in &self.node_coords {
            let mut distances: Vec<f32> = Vec::new();
            for coord2 in &self.node_coords {
                let distance: f32 =
                    ((coord2.x - coord.x).powi(2) + (coord2.y - coord.y).powi(2)).sqrt() as f32;
                distances.push(distance);
            }
            distance_matrix.push(distances);
        }
        distance_matrix
    }
}

impl std::str::FromStr for TspData {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut node_coords = Vec::<NodeCoord>::new();
        let v: Vec<&str> = s.split("\n").collect();
        for l in &v {
            node_coords.push(parse_node_coord_section_line(l.to_owned().trim())?);
        }
        Ok(TspData::new(node_coords))
    }
}

pub fn read(path: &str) -> Result<TspData, Box<dyn std::error::Error + 'static>> {
    let data: TspData = std::fs::read_to_string(path)?.parse()?;
    Ok(data)
}

fn parse_node_coord_section_line(line: &str) -> Result<NodeCoord, &'static str> {
    let parsed_line: Vec<f32> = line
        .split_whitespace()
        .map(|s| s.parse::<f32>().expect("Node coord must be numerical"))
        .collect();
    if parsed_line.len() < 3 {
        return Err("line too short");
    }
    Ok(NodeCoord::new(
        parsed_line.get(0).unwrap().to_owned() as i32,
        parsed_line.get(1).unwrap().to_owned(),
        parsed_line.get(2).unwrap().to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gr17() {
        let data = read(&"gr17.tsp").unwrap();
        let expected_edges = vec![
            vec![0, 633, 0, 257, 390, 0, 91, 661, 228, 0, 412, 227],
            vec![169, 383, 0, 150, 488, 112, 120, 267, 0, 80, 572, 196],
            vec![77, 351, 63, 0, 134, 530, 154, 105, 309, 34, 29, 0],
            vec![259, 555, 372, 175, 338, 264, 232, 249, 0, 505, 289, 262],
            vec![476, 196, 360, 444, 402, 495, 0, 353, 282, 110, 324, 61],
            vec![208, 292, 250, 352, 154, 0, 324, 638, 437, 240, 421, 329],
            vec![297, 314, 95, 578, 435, 0, 70, 567, 191, 27, 346, 83],
            vec![47, 68, 189, 439, 287, 254, 0, 211, 466, 74, 182, 243],
            vec![105, 150, 108, 326, 336, 184, 391, 145, 0, 268, 420, 53],
            vec![239, 199, 123, 207, 165, 383, 240, 140, 448, 202, 57, 0],
            vec![246, 745, 472, 237, 528, 364, 332, 349, 202, 685, 542, 157],
            vec![289, 426, 483, 0, 121, 518, 142, 84, 297, 35, 29, 36],
            vec![236, 390, 238, 301, 55, 96, 153, 336, 0],
        ];
    }

    #[test]
    fn dj38() {
        let data = read(&"dj38.tsp").unwrap();
        let expected_node_coords = vec![
            NodeCoord::new(1, 11003.611100, 42102.500000),
            NodeCoord::new(2, 11108.611100, 42373.888900),
            NodeCoord::new(3, 11133.333300, 42885.833300),
            NodeCoord::new(4, 11155.833300, 42712.500000),
            NodeCoord::new(5, 11183.333300, 42933.333300),
            NodeCoord::new(6, 11297.500000, 42853.333300),
            NodeCoord::new(7, 11310.277800, 42929.444400),
            NodeCoord::new(8, 11416.666700, 42983.333300),
            NodeCoord::new(9, 11423.888900, 43000.277800),
            NodeCoord::new(10, 11438.333300, 42057.222200),
            NodeCoord::new(11, 11461.111100, 43252.777800),
            NodeCoord::new(12, 11485.555600, 43187.222200),
            NodeCoord::new(13, 11503.055600, 42855.277800),
            NodeCoord::new(14, 11511.388900, 42106.388900),
            NodeCoord::new(15, 11522.222200, 42841.944400),
            NodeCoord::new(16, 11569.444400, 43136.666700),
            NodeCoord::new(17, 11583.333300, 43150.000000),
            NodeCoord::new(18, 11595.000000, 43148.055600),
            NodeCoord::new(19, 11600.000000, 43150.000000),
            NodeCoord::new(20, 11690.555600, 42686.666700),
            NodeCoord::new(21, 11715.833300, 41836.111100),
            NodeCoord::new(22, 11751.111100, 42814.444400),
            NodeCoord::new(23, 11770.277800, 42651.944400),
            NodeCoord::new(24, 11785.277800, 42884.444400),
            NodeCoord::new(25, 11822.777800, 42673.611100),
            NodeCoord::new(26, 11846.944400, 42660.555600),
            NodeCoord::new(27, 11963.055600, 43290.555600),
            NodeCoord::new(28, 11973.055600, 43026.111100),
            NodeCoord::new(29, 12058.333300, 42195.555600),
            NodeCoord::new(30, 12149.444400, 42477.500000),
            NodeCoord::new(31, 12286.944400, 43355.555600),
            NodeCoord::new(32, 12300.000000, 42433.333300),
            NodeCoord::new(33, 12355.833300, 43156.388900),
            NodeCoord::new(34, 12363.333300, 43189.166700),
            NodeCoord::new(35, 12372.777800, 42711.388900),
            NodeCoord::new(36, 12386.666700, 43334.722200),
            NodeCoord::new(37, 12421.666700, 42895.555600),
            NodeCoord::new(38, 12645.000000, 42973.333300),
        ];
    }
}
