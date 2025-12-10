use std::io::{self, Write};
use std::fmt::Display;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Display> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn add_first(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn add_last(&mut self, data: T) {
    let new_node = Box::new(Node { data, next: None });
    match self.head.as_mut() {
        None => self.head = Some(new_node),
        Some(_) => {
            let mut cur = self.head.as_mut().unwrap();
            while cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next = Some(new_node);
        }
    }
}

    pub fn add_at(&mut self, index: usize, data: T) {
        if index == 0 {
            self.add_first(data);
            return;
        }
        let mut cur = &mut self.head;
        for _ in 0..index - 1 {
            match cur {
                Some(node) => cur = &mut node.next,
                None => {
                    println!("Index out of bounds.");
                    return;
                }
            }
        }
        match cur {
            Some(node) => {
                let new_node = Box::new(Node {
                    data,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
            }
            None => println!("Index out of bounds."),
        }
    }

    pub fn delete_first(&mut self) {
        if self.head.is_none() {
            println!("List is empty.");
            return;
        }
        self.head = self.head.take().and_then(|node| node.next);
    }

    pub fn delete_last(&mut self) {
        if self.head.is_none() {
            println!("List is empty.");
            return;
        }
        if self.head.as_ref().unwrap().next.is_none() {
            self.head = None;
            return;
        }
        let mut cur = self.head.as_mut().unwrap();
        while cur.next.as_ref().unwrap().next.is_some() {
            cur = cur.next.as_mut().unwrap();
        }
        cur.next = None;
    }

    pub fn delete_at(&mut self, index: usize) {
        if self.head.is_none() {
            println!("Invalid operation.");
            return;
        }
        if index == 0 {
            self.delete_first();
            return;
        }
        let mut cur = &mut self.head;
        for _ in 0..index - 1 {
            match cur {
                Some(node) => cur = &mut node.next,
                None => {
                    println!("Index out of bounds.");
                    return;
                }
            }
        }
        match cur {
            Some(node) => {
                if node.next.is_none() {
                    println!("Index out of bounds.");
                } else {
                    node.next = node.next.take().and_then(|n| n.next);
                }
            }
            None => println!("Index out of bounds."),
        }
    }

    pub fn print_list(&self) {
        if self.head.is_none() {
            println!("List is empty");
            return;
        }
        let mut cur = self.head.as_ref();
        while let Some(node) = cur {
            print!("{} -> ", node.data);
            cur = node.next.as_ref().map(|b| b);
        }
        println!("null");
    }

    pub fn reverse(&mut self) {
        let mut prev: Option<Box<Node<T>>> = None;
        let mut curr = self.head.take();
        while let Some(mut boxed) = curr {
            let next = boxed.next.take();
            boxed.next = prev.take();
            prev = Some(boxed);
            curr = next;
        }
        self.head = prev;
    }
}

fn read_i32(prompt: &str) -> Option<i32> {
    print!("{}", prompt);
    io::stdout().flush().ok()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok()?;
    s.trim().parse::<i32>().ok()
}

fn read_usize(prompt: &str) -> Option<usize> {
    read_i32(prompt).and_then(|v| if v >= 0 { Some(v as usize) } else { None })
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    println!("--- Linked List Menu ---");

    loop {
        println!("\nChoose an operation:");
        println!("1. Add First");
        println!("2. Add Last");
        println!("3. Add At Position");
        println!("4. Delete First");
        println!("5. Delete Last");
        println!("6. Delete At Position");
        println!("7. Print List");
        println!("8. Reverse List");
        println!("9. Exit");

        let choice = match read_i32("Enter choice: ") {
            Some(v) => v,
            None => {
                println!("Invalid input.");
                continue;
            }
        };

        match choice {
            1 => {
                if let Some(data) = read_i32("Enter number to add at start: ") {
                    list.add_first(data);
                } else {
                    println!("Invalid number.");
                }
            }
            2 => {
                if let Some(data) = read_i32("Enter number to add at end: ") {
                    list.add_last(data);
                } else {
                    println!("Invalid number.");
                }
            }
            3 => {
                if let Some(index) = read_usize("Enter index: ") {
                    if let Some(data) = read_i32("Enter number: ") {
                        list.add_at(index, data);
                    } else {
                        println!("Invalid number.");
                    }
                } else {
                    println!("Invalid index.");
                }
            }
            4 => {
                list.delete_first();
                println!("Deleted first node.");
            }
            5 => {
                list.delete_last();
                println!("Deleted last node.");
            }
            6 => {
                if let Some(index) = read_usize("Enter index to delete: ") {
                    list.delete_at(index);
                } else {
                    println!("Invalid index.");
                }
            }
            7 => {
                print!("Current List: ");
                list.print_list();
            }
            8 => {
                list.reverse();
                println!("List reversed successfully!");
            }
            9 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }

        // After each operation (except explicit print) show list
        if choice != 7 {
            print!("Current List: ");
            list.print_list();
        }
    }
}