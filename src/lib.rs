use core::fmt;
use std::{thread, time, io::{Write, stdout}};

// macro_rules! get_input {
//     ($t:ty) => {
//         {
//             let mut s = String::new();
//             std::io::stdin().read_line(&mut s).unwrap();
//             s.trim().parse::<$t>().unwrap()
//         }
//     }
// }

#[derive(Default)]
#[derive(Clone)]
pub struct Item {
    id: &'static str,
    examine: &'static str,
    first_glance: &'static str,
    container: bool,
    open: Option<bool>,
    open_message: Option<&'static str>,
    close_message: Option<&'static str>,
    contains: Option<Vec<Item>>,
}

pub struct Player {
    inventory: Vec<Item>,
    hand: Option<Item>
}

pub enum Directions {
    North,
    East,
    South,
    West
}

pub struct Exit<'a> {
    direction: Directions,
    room: &'a Room<'a>
}

#[derive(Default)]
pub struct Room<'a> {
    first_glance: &'static str,
    contains: (Vec<Item>, Option<&'a Player>),
    exits: Vec<Exit<'a>>
}


impl fmt::Display for Exit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.direction {
            Directions::North => write!(f, "North"),
            Directions::East => write!(f, "East"),
            Directions::South => write!(f, "South"),
            Directions::West => write!(f, "West"),
        }
    }
}


fn typewriter(text: &str) {
    for i in text.chars() {
        print!("{}", i);
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(20));
    }
    println!();
}

pub fn room_enter(room: Room) {
    typewriter(room.first_glance);
    for i in room.contains.0 {
        typewriter(i.first_glance);
    }
    let room_exit_count = room.exits.len();
    for i in room.exits {
        if room_exit_count >= 1 {
            typewriter(format!("There is an exit to the {}", i).as_str());
        }
    }
}

pub fn examine_item(item: Item) {
    typewriter(item.examine);
}

pub fn view_inventory<'a>(player: &'a Player) {
    for i in &player.inventory {
        typewriter(format!("id: {}, {}", i.id, i.first_glance).as_str());
    }
    if player.hand.is_some() {
        typewriter(format!("In your hand is: {}", player.hand.clone().unwrap().id).as_str());
    }
    println!();
}

pub fn move_item_to_hand<'a>(player: &'a mut Player, id: &str) {
    for i in &player.inventory {
        if i.id == id {
            player.hand = Some(i.to_owned());
            player.inventory.remove(player.inventory.iter().position(|f| f.id == i.id).unwrap());
            return;
        }
    }
    typewriter("There isn't an item with that id!");
}

#[cfg(test)]
mod tests {
    use crate::{Item, room_enter, Room, Exit, Directions, Player, view_inventory, move_item_to_hand};

    #[test]
    fn test_room() {

        let note = Item{
            id: "Note",
            examine: "It says, \"This is a test note, for testing.\"",
            first_glance: "There is a note in the center of the room.",
            container: false,
            ..Item::default()
        };

        let test_exit_room = Room {
            ..Room::default()
        };
        let room = Room{
            first_glance: "You come to a room that has a grid like texure on the walls, there is the word \"Test\" in a big, arial font.",
            contains: (vec![note], None),
            exits: vec![
                Exit {
                    direction: Directions::North,
                    room: &test_exit_room
                },
                Exit {
                    direction: Directions::South,
                    room: &test_exit_room
                }
            ]
        };
        room_enter(room);
    }
    
    #[test]
    fn player_test() {
        let mut player = Player {
            inventory: vec![
                Item {
                    id: "stick",
                    first_glance: "A testing stick",
                    ..Item::default()
                },
                Item {
                    id: "slab",
                    first_glance: "A testing slab",
                    ..Item::default()
                }
            ],
            hand: None
        };
        
        let Room = Room {
            contains: (vec![], Some(&player)),
            ..Room::default()
        };

        view_inventory(&player);
        move_item_to_hand(&mut player, "stick");
        view_inventory(&player);
        // drop_held_item(&mut room, &mut player)
        // view_inventory(&player);
    }
}
