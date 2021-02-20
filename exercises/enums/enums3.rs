// enums3.rs
// Address all the TODOs to make the tests pass!

struct Point {
    x: u8,
    y: u8,
}

#[derive(PartialEq, Debug)]
struct ColorRGB(u8, u8, u8);

enum Message {
    Move(Point),
    Echo(String),
    ChangeColor(ColorRGB),
    Quit,
}

struct State {
    color: ColorRGB,
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: ColorRGB) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::Move(point) => self.move_position(point),
            Message::Echo(text) => self.echo(text),
            Message::ChangeColor(color) => self.change_color(color),
            Message::Quit => self.quit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: ColorRGB(0, 0, 0),
        };
        state.process(Message::ChangeColor(ColorRGB(255, 0, 255)));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, ColorRGB(255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
    }
}
