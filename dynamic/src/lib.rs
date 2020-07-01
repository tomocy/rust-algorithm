use std::collections::HashMap;

#[allow(dead_code)]
fn dynamic(size: u32, items: Vec<Item>) -> f32 {
    let col = prepare_column(size, &items);

    items
        .iter()
        .fold(Record::new(None), |mut record, item| {
            record.fill(&col, item);
            Record::new(Some(Box::new(record.clone())))
        })
        .get_prev_price(col.end)
}

fn prepare_column(size: u32, items: &Vec<Item>) -> Column {
    let size = size as f32;

    let base = items
        .iter()
        .map(|item| item.weight.fract())
        .filter(|&base| base != 0.0)
        .fold(1.0, |a: f32, b| a.min(b));
    let start = items
        .iter()
        .map(|item| item.weight)
        .fold(size, |a: f32, b| a.min(b));

    Column::new(base, start, size)
}

#[derive(Debug, Clone)]
struct Record {
    prev: Option<Box<Record>>,
    prices: HashMap<String, f32>,
}

impl Record {
    fn new(prev: Option<Box<Record>>) -> Self {
        Self {
            prev,
            prices: HashMap::new(),
        }
    }

    fn fill(&mut self, col: &Column, item: &Item) {
        col.for_base(|cap| {
            if cap < item.weight {
                self.set_price(cap, self.get_prev_price(cap));
                return;
            }

            let preliminary_price = item.price + self.get_prev_price(cap - item.weight);
            self.set_price(cap, f32::max(preliminary_price, self.get_prev_price(cap)));
        });
    }

    fn get_prev_price(&self, key: f32) -> f32 {
        match &self.prev {
            Some(prev) => match prev.get_price(key) {
                Some(x) => x,
                None => 0.0,
            },
            None => 0.0,
        }
    }

    fn get_price(&self, key: f32) -> Option<f32> {
        match self.prices.get(&key.to_string()) {
            Some(&x) => Some(x),
            None => None,
        }
    }

    fn set_price(&mut self, key: f32, price: f32) {
        self.prices.insert(key.to_string(), price);
    }
}

struct Column {
    base: f32,
    start: f32,
    end: f32,
}

impl Column {
    fn new(base: f32, start: f32, end: f32) -> Self {
        Self { base, start, end }
    }

    fn for_base<T>(&self, mut invoke: T)
    where
        T: FnMut(f32),
    {
        let mut i = self.start;
        while i <= self.end {
            invoke(i);
            i += self.base;
        }
    }
}

struct Item {
    price: f32,
    weight: f32,
}

impl Item {
    #[allow(dead_code)]
    fn new(price: f32, weight: f32) -> Self {
        Self { price, weight }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            3500.0,
            dynamic(
                4,
                vec![
                    Item::new(2000.0, 3.0),
                    Item::new(1500.0, 1.0),
                    Item::new(3000.0, 4.0),
                ]
            )
        );
    }
}
