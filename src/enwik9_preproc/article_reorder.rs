use std::io::BufRead;
use std::io::Write;

const NUM_OF_ARTICLES: usize = 243425;

struct Accumulator {
    id: i32,
    start: i32,
    end: i32,
}

enum ParserState {
    ExpectPage = 0,
    ExpectId,
    ExpectPageEnd,
}

pub fn action_get_line_count(s: String) -> i32 {
    s.lines().count() as i32
}

pub fn action_get_id(s: String) -> i32 {
    let mut s = s.replace(" ", "");
    let tok = "<id>";
    if let Some(i) = s.find(tok) {
        s.replace_range(i..(i + tok.len()), "");
    }
    let tok = "</id>";
    if let Some(i) = s.find(tok) {
        s.replace_range(i..(i + tok.len()), "");
    }
    s.parse().unwrap_or(0)
}

pub fn save_pagestart(res: i32, acc: &mut Accumulator) {
    acc.start = res;
}

pub fn save_id(res: i32, acc: &mut Accumulator) {
    acc.id = res;
}

pub fn save_pageend(res: i32, acc: &mut Accumulator) {
    acc.end = res;
}

pub fn bubblesort(mylist: &mut Vec<Accumulator>) {
    for i in 1..mylist.len() {
        for j in 0..(mylist.len() - i) {
            if mylist[j].id > mylist[j + 1].id {
                mylist.swap(j, j + 1);
            }
        }
    }
}

pub fn reorder() -> i32 {
    let mut line_count = 0;

    let file = std::fs::File::open(".main").unwrap();
    let order_file = std::fs::File::open(".new_article_order").unwrap();

    let mut lines = Vec::new();
    let mut positions = Vec::new();

    let patterns = vec!["<page>", "<id>", "</page>"];
    let transitions = vec![
        ParserState::ExpectId,
        ParserState::ExpectPageEnd,
        ParserState::ExpectPage,
    ];
    let actions: [fn(String) -> i32; 3] =
        [action_get_line_count, action_get_id, action_get_line_count];
    let save: [fn(i32, &mut Accumulator); 3] =
        [save_pagestart, save_id, save_pageend];

    let mut state = ParserState::ExpectPage;

    let mut vec = Vec::new();

    let mut s = String::new();
    let mut pattern = String::new();
    let mut res = 0;
    let mut acc = Accumulator {
        id: 0,
        start: 0,
        end: 0,
    };
    let mut reader = std::io::BufReader::new(file);

    while reader.read_line(&mut s).unwrap() > 0 {
        pattern = patterns[state as usize].to_string();
        if s.contains(&pattern) {
            res = actions[state as usize](s.clone());
            save[state as usize](res, &mut acc);
            state = match transitions[state as usize] {
                ParserState::ExpectPage => ParserState::ExpectPage,
                ParserState::ExpectId => ParserState::ExpectId,
                ParserState::ExpectPageEnd => ParserState::ExpectPageEnd,
            };
            if let ParserState::ExpectPage = state {
                vec.push(acc);
            }
        }
        line_count += 1;
        lines.push(s.clone());
        s.clear();
    }

    let mut used = vec![0; NUM_OF_ARTICLES];
    let mut order_reader = std::io::BufReader::new(order_file);
    while order_reader.read_line(&mut s).unwrap() > 0 {
        positions.push(s.trim().parse().unwrap());
        used[s.trim().parse::<usize>().unwrap()] = 1;
        s.clear();
    }

    if positions.len() < NUM_OF_ARTICLES {
        for i in 0..NUM_OF_ARTICLES {
            if used[i] == 0 {
                positions.push(i);
            }
        }
    }

    let mut out = std::fs::File::create(".main_reordered").unwrap();
    for pos in positions {
        for j in vec[pos].start..=vec[pos].end {
            let usize_j = j as usize;
            writeln!(out, "{}", lines[usize_j]).unwrap();
        }
    }

    return 0;
}

pub fn sort() -> i32 {
    let mut line_count = 0;

    let file = match std::fs::File::open(".main_decomp_restored") {
        Ok(file) => file,
        Err(_) => return -1,
    };

    let mut lines = Vec::new();

    let patterns = vec!["<page>", "<id>", "</page>"];
    let transitions = vec![
        ParserState::ExpectId,
        ParserState::ExpectPageEnd,
        ParserState::ExpectPage,
    ];
    let actions: [fn(String) -> i32; 3] =
        [action_get_line_count, action_get_id, action_get_line_count];
    let save: [fn(i32, &mut Accumulator); 3] =
        [save_pagestart, save_id, save_pageend];

    let mut state = ParserState::ExpectPage;

    let mut vec = Vec::new();

    let mut s = String::new();
    let mut pattern = String::new();
    let mut res = 0;
    let mut acc = Accumulator {
        id: 0,
        start: 0,
        end: 0,
    };
    let mut reader = std::io::BufReader::new(file);
    while reader.read_line(&mut s).unwrap() > 0 {
        pattern = patterns[state as usize].to_string();
        if s.contains(&pattern) {
            res = actions[state as usize](s.clone());
            save[state as usize](res, &mut acc);
            state = transitions[state as usize];
            if let ParserState::ExpectPage = state {
                vec.push(acc);
            }
        }
        line_count += 1;
        lines.push(s.clone());
        s.clear();
    }

    bubblesort(&mut vec);

    let mut out = match std::fs::File::create(".main_decomp_restored_sorted") {
        Ok(file) => file,
        Err(_) => return -1,
    };
    for acc in vec {
        for j in acc.start..=acc.end {
            let usize_j = j as usize;
            writeln!(out, "{}", lines[usize_j]).unwrap();
        }
    }

    return 0;
}
