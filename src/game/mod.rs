#[derive(Copy, Clone)]
pub enum FieldType {
    Water,
    
    Food,
    
    Land,
    
    Dead,
    
    MyAnt,
    
    EnemyAnt
}

struct UpdateCommand {
    update_type: String,
    x: i64,
    y: i64,
    owner: Option<i64>
}

#[derive(Copy, Clone)]
pub struct Tile {
    visible: bool,
    field_type: FieldType
}

pub struct Board {
    tiles: [[Tile; 256]; 256]
}


impl Board {
    pub fn new() -> Board {
        Board{tiles: [[Tile{visible: false, field_type: FieldType::Land}; 256]; 256]}
    }

    pub fn update(&mut self, updateCommnd: String) {

    }
}

fn parse_update_command(line: String) -> UpdateCommand {
    let mut split = line.trim().split(" ");
    let update_type = split.next().unwrap();
    let x = split.next().unwrap();
    let y = split.next().unwrap();
    let owner = split.next().map(|i| i.parse::<i64>().unwrap()); 
    UpdateCommand{
        update_type: update_type.to_string(), 
        x:x.parse::<i64>().unwrap(), 
        y: y.parse::<i64>().unwrap(), 
        owner: owner
    }
}
