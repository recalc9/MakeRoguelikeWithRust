use rltk::{Rltk,GameState,RGB,VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max,min};
use specs_derive::Component;

mod map;
pub use map::*;
mod rect;
pub use rect::Rect;

#[derive(Component)]
struct Position{
    x: i32,
    y: i32,
}
 
#[derive(Component)]
struct Renderable{
    glyph: rltk::FontCharType,
    fg: RGB,
    bg:  RGB,
}



fn draw_map(map: &[TileType],ctx: &mut Rltk){
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter(){
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
            TileType::Wall =>{
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        x += 1;
        if x > 79{
            x = 0;
            y += 1;
        }
    }
}

#[derive(Component,Debug)]
struct  Player{}

fn try_move_player(delta_x:i32,delta_y:i32,ecs: &mut World){
    let mut position = ecs.write_storage::<Position>();
    let mut player = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player,pos) in (&mut player,&mut position).join(){
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall{
            pos.x = min(79, max(0, pos.x+delta_x));
            pos.y = min(49, max(0, pos.y+delta_y));
        }
    }
}

fn player_input(gs: &mut State,ctx: &mut Rltk){
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

struct State{
    ecs: World
}

impl State {
    fn run_systems(&mut self){
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
        self.run_systems();
        player_input(self, ctx);
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);
        let position = self.ecs.read_storage::<Position>();
        let renderable = self.ecs.read_storage::<Renderable>();
        for (pos,render) in (&position,&renderable).join(){
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError{
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State{
        ecs:World::new()
    };
     
    gs.ecs.insert(new_map_rooms_and_corridors());
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    
    gs.ecs
        .create_entity()
        .with(Position{x:40,y:25})
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .build();

    rltk::main_loop(context, gs)
}
