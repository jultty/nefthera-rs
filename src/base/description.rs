#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Description {
    text: &'static str,
}

impl Description {
    pub fn instantiate(text: &'static str) -> Description {
        if text.len() < 350 {
            Description { text }
        } else {
            panic!("Description text can't exceed 350 bytes");
        }
    }

    pub fn get(self) -> &'static str {
        self.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn description_can_be_set() {
        let _description = Description::instantiate("A small cabin in the woods, found after following a barely visible path that strays from the main road to the larger Ibonhaun Academy, leading through the Oppos Woods to a clearing where the original laboratory of Alexander Ibonhaun still could be reached.");
    }

    #[test]
    fn description_text_can_be_read() {
        let description = Description::instantiate("A small cabin in the woods, found after following a barely visible path that strays from the main road to the larger Ibonhaun Academy.");
        let _read_description = description.get();
    }

    #[test]
    #[should_panic]
    fn description_cant_exceed_300_bytes() {
        let _description = Description::instantiate("A small cabin in the woods, found after following a barely visible path that strays from the main road to the larger Ibonhaun Academy, leading through the Oppos Woods to a clearing where the original laboratory of Alexander Ibonhaun still could be reached. A modest wooden structure with strong, ever-changing smells. All walls were dotted with the colors and the glimmer of flasks, boxes, and objects of curious shapes that seemed as if every possible substance from the lands of Fiji could be found there.");
    }
}
