use itertools::Itertools;

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

fn hash_s(s: &str) -> u8 {
    let mut h: u8 = 0;
    for c in s.bytes() {
        h = h.wrapping_add(c);
        h = h.wrapping_mul(17);
    }
    h
}

enum Operaion {
    Replace,
    Remove,
}

struct Step {
    data: String,
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        Self {
            data: value.to_string(),
        }
    }
}

impl Step {
    fn label(&self) -> &str {
        self.data.split(['-', '=']).next().unwrap()
    }

    fn box_num(&self) -> u8 {
        hash_s(self.label())
    }

    fn operation(&self) -> Operaion {
        let i = self.data.find(['-', '=']).unwrap();

        match self.data.as_bytes()[i] {
            b'=' => Operaion::Replace,
            b'-' => Operaion::Remove,
            _ => panic!("Cannot parse operation"),
        }
    }

    fn focal_length(&self) -> Option<u32> {
        if let Operaion::Replace = self.operation() {
            Some(self.data.split_once('=').unwrap().1.parse().unwrap())
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Clone, Default)]
struct Box {
    list: Vec<Lens>,
}

impl Box {
    fn remove_lens_by_label(&mut self, label: &str) {
        if let Some((pos, _)) = self.list.iter().find_position(|lens| lens.label == label) {
            self.list.remove(pos);
        }
    }

    fn replace_lens(&mut self, new_lens: Lens) {
        if let Some(lens) = self
            .list
            .iter_mut()
            .find(|lens| lens.label == new_lens.label)
        {
            lens.focal_length = new_lens.focal_length;
        } else {
            self.list.push(new_lens);
        }
    }
}

fn parse_steps() -> Vec<Step> {
    let input = include_str!("../../puzzle_input/d15").trim();
    let steps = input.replace('\n', "");
    steps.split(',').map(|s| s.into()).collect_vec()
}

fn part1() {
    let steps = parse_steps();
    let hash_total = steps
        .into_iter()
        .fold(0, |total, step| total + hash_s(&step.data) as u64);
    println!("{}", hash_total);
}

fn part2() {
    let steps = parse_steps();
    let mut boxes = vec![Box::default(); 256];

    for step in steps {
        match step.operation() {
            Operaion::Replace => {
                let lens = Lens {
                    label: step.label().to_string(),
                    focal_length: step.focal_length().unwrap(),
                };
                boxes[step.box_num() as usize].replace_lens(lens);
            }
            Operaion::Remove => {
                boxes[step.box_num() as usize].remove_lens_by_label(step.label());
            }
        }
    }

    let mut total: usize = 0;
    for (box_i, lens_box) in boxes.iter().enumerate() {
        let box_num = box_i + 1;
        for (slot_i, lens) in lens_box.list.iter().enumerate() {
            let slot_num = slot_i + 1;
            total += box_num * slot_num * lens.focal_length as usize;
        }
    }

    println!("{}", total);
}
