use super::Rect;

#[derive(PartialEq,Clone, Copy)]
pub enum  TileType {
    Wall,Floor
}

pub fn xy_idx(x:i32 , y:i32) -> usize{
    (y as usize * 80) + x as usize
}




///创建一个有实线边界和400个随机放置墙壁的地图。没有人能保证它不会
///看起来很糟糕。
pub fn new_map_test() -> Vec<TileType>{
    let mut map = vec![TileType::Floor;80*50];

    for x in 0..80 {
        map[xy_idx(x,0)] = TileType::Wall;
        map[xy_idx(x,49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25){
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType>{
    let mut map = vec![TileType::Wall;80*50];

    let room1 = Rect::new(20,15,10,15);
    let room2 = Rect::new(35,15,10,15);

    apply_room_to_map(&room1,&mut map);
    apply_room_to_map(&room2,&mut map);

    map
}

fn apply_room_to_map(room: &Rect,map:&mut [TileType]){
    for y in room.y1 + 1 ..= room.y2{
        for x in room.x1 + 1 ..= room.x2{
            map[xy_idx(x,y)] = TileType::Floor;
        }
    }
}