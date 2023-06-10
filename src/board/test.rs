use super::*;

mod position {
    use super::*;

    #[test]
    fn apply_up() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&Up, 1);
        assert_eq!(pos, BoardPosition { x: 1, y: 2 });
    }

    #[test]
    fn apply_down() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&Down, 1);
        assert_eq!(pos, BoardPosition { x: 1, y: 0 });
    }

    #[test]
    fn apply_left() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&Left, 1);
        assert_eq!(pos, BoardPosition { x: 0, y: 1 });
    }

    #[test]
    fn apply_right() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&Right, 1);
        assert_eq!(pos, BoardPosition { x: 2, y: 1 });
    }

    #[test]
    fn apply_up_left() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&UpLeft, 1);
        assert_eq!(pos, BoardPosition { x: 0, y: 2 });
    }

    #[test]
    fn apply_up_right() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&UpRight, 1);
        assert_eq!(pos, BoardPosition { x: 2, y: 2 });
    }

    #[test]
    fn apply_down_left() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&DownLeft, 1);
        assert_eq!(pos, BoardPosition { x: 0, y: 0 });
    }

    #[test]
    fn apply_down_right() {
        use super::Direction::*;
        let mut pos = BoardPosition { x: 1, y: 1 };
        pos.apply_direction(&DownRight, 1);
        assert_eq!(pos, BoardPosition { x: 2, y: 0 });
    }
}
