#[derive(Debug, Clone)]
struct Product {
    name: String,
    category: String,
    price: f64,
}
impl Product {
    fn new(name: &str, category: &str, price: f64) -> Self {
        Product {
            name: name.to_string(),
            category: category.to_string(),
            price,
        }
    }
}

struct FilterPipeline<T> {
    filters: Vec<Box<dyn Fn(&T) -> bool>>,
}

impl<T> FilterPipeline<T> {
    fn new() -> Self {
        FilterPipeline {
            filters: Vec::new(),
        }
    }

    fn add<F>(&mut self, filter: F)
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.filters.push(Box::new(filter));
    }

    fn apply<'a>(&self, items: &'a [T]) -> Vec<&'a T> {
        items
            .iter()
            .filter(|item| self.filters.iter().all(|pred| pred(item)))
            .collect()
    }
}

fn main() {
    let products = vec![
        Product::new("Apple iPhone", "Electronics", 999.99),
        Product::new("Banana", "Food", 0.99),
        Product::new("Organic Banana", "Food", 1.29),
        Product::new("C++ Book", "Books", 39.99),
    ];

    let cheap_foods: Vec<&Product> = products
        .iter()
        .filter(|p| p.price < 20.0 && p.category == "Food")
        .collect();

    println!("Cheap Foods: {:#?}", cheap_foods);

    let mut pipeline = FilterPipeline::new();

    pipeline.add(|p: &Product| p.price < 20.0);
    pipeline.add(|p: &Product| p.category == "Food");

    let pipeline_filtered: Vec<&Product> = pipeline.apply(&products);

    println!("Pipeline Filtered Cheap Foods: {:#?}", pipeline_filtered)
}
