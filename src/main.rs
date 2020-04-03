mod alphagram;
mod priority;
use priority_queue::PriorityQueue;

fn main() {
    let string = "Mañana".to_string();
    let chars = alphagram::new(&string[..]);
    for c in chars {
        println!("char is {}", c);
    }

    let mut pq = PriorityQueue::new();

    assert!(pq.is_empty());
    pq.push("Apples", (1, 1));
    pq.push("Bananas", (1, 2));
    pq.push("Strawberries", (2, 3));
    let res = pq.pop();
    println!("we got {:?}", res);
}
