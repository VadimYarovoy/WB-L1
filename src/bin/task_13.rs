// На вход, через стандартный поток ввода, поступает последовательность строк,
// строки могут повторяться. Необходимо вывести в стандартный поток вывода строки,
// исключив повторения, не используя std::collections::*.

use std::io;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let num_lines = lines.next().unwrap().unwrap().parse::<usize>().expect("Must be number");

    // Префиксное дерево было использовано по ряду причин
    // 1) Простота реализации
    // 2) Эффективное использование памяти при наличии дубликатов или подстрок
    // 3) Производительность зависит только от дины слова

    // Конечно лучшим с точки зрения производительности была бы HashMap,
    // но есть ряд моментов, по которым было решено ее не реализовывать
    // 1) Сложность - Во первых, нужно реализовать свой Vec,
    //    это вохможно но все будет в unsafe и без должного доказательста нельзя гарантировать то,
    //    что он будет безопасен. Во вторых, нужно реализовывать работу с колизиями, в std::collections::HashMap
    //    используется метод - open addressing, это не так сложно как свой Vec. Наконец, нужна функция хеширования, но
    //    ее можно взять из стандартной библиотеки
    // 2) Производительность собственной HashMap Может оказаться не O(1) - средний случай, а O(n) из-за перечисленных в пункте 1 проблем.
    let mut trie = Trie::new();

    // Связанный список с интейфейсом сека оптимален, так как все опрерации с ним - O(1)
    let mut list = MyLinkedList::new();

    for _ in 0..num_lines {
        let s = lines.next().unwrap().unwrap();

        if !trie.search(&s) {
            trie.insert(s.clone());
            list.add_at_head(s);
        }
    }

    println!("Unique strings:");
    let mut idx = 1;
    while let Some(val) = list.get_at_head() {
        println!("{}) {}", idx, val);
        list.delete_at_head();
        idx += 1;
    }
}

// Для хранения уникальных строк используем префиксное дерево
#[derive(Default)]
struct Trie {
    root: TrieNode,
}

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_terminal: bool,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn offset(c: char) -> usize {
        (c as usize) - ('a' as usize)
    }

    // O(m), m - длинна слова
    fn insert(&mut self, word: String) {
        let mut current = &mut self.root;
        for c in word.chars() {
            current = current.children[Self::offset(c)].get_or_insert_with(Box::default);
        }

        current.is_terminal = true;
    }

    fn get_node(&self, word: &String) -> Option<&TrieNode> {
        let mut current = &self.root;
        for c in word.chars() {
            match current.children[Self::offset(c)] {
                Some(ref node) => current = node,
                None => return None,
            }
        }
        Some(current)
    }

    // O(m), m - длинна слова
    fn search(&self, word: &String) -> bool {
        if let Some(node) = self.get_node(word) {
            node.is_terminal
        } else {
            false
        }
    }
}

// Для хранения строк используется связвнный список с интерфейсом стека
struct MyLinkedList {
    len: i32,
    root: Option<Box<MyLinkedNode>>,
}

struct MyLinkedNode {
    val: String,
    next: Option<Box<MyLinkedNode>>,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList { len: 0, root: None }
    }

    // O(1)
    fn get_at_head(&self) -> Option<String> {
        if self.len == 0 {
            return None;
        }
        let h = self.root.as_ref().unwrap();
        return Some(h.val.clone());
    }

    // O(1)
    fn add_at_head(&mut self, val: String) {
        let r = self.root.take();
        self.root = Some(Box::new(MyLinkedNode { val: val, next: r }));
        self.len += 1;
    }

    // O(1)
    fn delete_at_head(&mut self) {
        if self.len == 0 {
            return;
        }
        match self.len {
            1 => self.root = None,
            _ => self.root = self.root.as_mut().unwrap().next.take(),
        }
        self.len -= 1;
    }
}
