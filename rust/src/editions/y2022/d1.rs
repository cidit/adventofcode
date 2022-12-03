pub fn p1(input: &str) -> String {
    let lunch_boxes = parse(input);
    let mut calories_per_box = lunch_boxes.iter()
        .map(LunchBox::total_calories)
        .enumerate()
        .collect::<Vec<_>>();
    calories_per_box.sort_by_key(|&(_, calories)| calories);
    let (_, num_calories) = calories_per_box.last().unwrap();
    (num_calories).to_string()           
}

pub fn p2(input: &str) -> String {
    let lunch_boxes = parse(input);
    let mut calories_per_box = lunch_boxes.iter()
        .map(LunchBox::total_calories)
        .enumerate()
        .collect::<Vec<_>>();
    calories_per_box.sort_by_key(|&(_, calories)| calories);
    let total_calories_top_3: i32 = calories_per_box.iter().rev().take(3).map(|(_, calories)| calories).sum();
    (total_calories_top_3).to_string()    
}

fn parse(input: &str) -> Vec<LunchBox> {
    let calories_per_item: Vec<_> = input.split("\n")
        .map(str::trim)
        .map(str::parse::<i32>)
        .collect();
    let num_lunch_boxes = calories_per_item.iter().filter(|r| r.is_err()).count() + 1;
    let mut calories_per_item_iter = calories_per_item.into_iter();
    (0..num_lunch_boxes)
        .map(|_| calories_per_item_iter
                                .by_ref()
                                .take_while(|r| r.is_ok())
                                .flatten()
                                .collect()
            )
        .map(|v| LunchBox { food_calories: v})
        .collect()
}

#[derive(Debug)]
struct LunchBox {
    food_calories: Vec<i32>
}

impl LunchBox {
    fn total_calories(&self) -> i32 {
        self.food_calories.iter().sum()
    }
}
