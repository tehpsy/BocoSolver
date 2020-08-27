use super::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_available_moves() {
        let node2 = Node2{
            height:2,
            width:2,
            player:Player{position:Position{x:0, y:0}},
            reds:Reds{
                small:Unit{position:Position{x:0, y:1}, orientation:Orientation::Up, color:Color::Red, size:Size::Small},
                large:Unit{position:Position{x:1, y:1}, orientation:Orientation::Left, color:Color::Red, size:Size::Large},
            }
        };
    
        let available_moves = node2.available_moves();

        assert_eq!(available_moves, [Orientation::Right, Orientation::Down]);
    }
}