pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        self.label.chars().count()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str(&self.label).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let s = &self.label.label;
        let width = self.width();
        buffer.write_str(&format!("+{}+\n", "-".repeat(width-2))).unwrap();
        buffer.write_str(&format!("| {} |\n", s)).unwrap();
        buffer.write_str(&format!("+{}+\n", "-".repeat(self.width()-2))).unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        let max_width = self.widgets
            .iter()
            .map(|w| w.width())
            .max()
            .unwrap_or_default();
        
        max_width + 2
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        let top = "+".to_owned() + &"-".repeat(self.width()) + "+\n";
        let mid = "|".to_owned() + &format!("{: ^width$}", self.title) + "|\n";
        let bottom = "+".to_owned() + &"=".repeat(self.width()) + "+\n";
        buffer.write_str(&top).unwrap();
        buffer.write_str(&mid).unwrap();
        buffer.write_str(&bottom).unwrap();

        for widget in &self.widgets {
            let mut buf = String::new();
            widget.draw_into(&mut buf);

            for line in buf.lines() {
                let pad = self.inner_width() - widget.width() + 1;
                writeln!(buffer, "| {} {: >pad$}", line, "|").unwrap();
            }
            
        };
        buffer.write_str(&top).unwrap();
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}