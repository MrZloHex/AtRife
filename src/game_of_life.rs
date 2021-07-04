pub struct Game {
    field: [[u8; 8]; 8]
}

impl Game {
    pub fn new(start: [[u8; 8]; 8]) -> Game {
        Game {
            field: start
        }
    }

    pub fn get_field(&mut self) -> [[u8; 8]; 8] {
        self.field.clone()
    }

    pub fn make_step(&mut self) {
        let in_grid = self.get_field();
        let mut out_grid: [[u8;8];8] = [[0;8];8];
        let mut r: i8 = 0;
        for row in in_grid.iter() {
            let mut c: i8 = 0;
            for item in row.iter() {
                let mut neighbor: u8 = 0;

                if *item == 1 {
                    // ALIVE
                    match in_grid.get((r-1) as usize) {
                        Some(col) => { match col.get((c-1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r-1) as usize) {
                        Some(col) => { match col.get((c) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r-1) as usize) {
                        Some(col) => { match col.get((c+1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r) as usize) {
                        Some(col) => { match col.get((c-1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r) as usize) {
                        Some(col) => { match col.get((c+1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r+1) as usize) {
                        Some(col) => { match col.get((c-1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r+1) as usize) {
                        Some(col) => { match col.get((c) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r+1) as usize) {
                        Some(col) => { match col.get((c+1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }

                    if neighbor == 2 || neighbor == 3 {
                        out_grid[r as usize][c as usize] = 1;
                    }
                    else {
                        out_grid[r as usize][c as usize] = 0;
                    }
                }
                else if *item == 0 {
                    // DEAD

                    match in_grid.get((r-1) as usize) {
                        Some(col) => { match col.get((c-1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r-1) as usize) {
                        Some(col) => { match col.get((c) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r-1) as usize) {
                        Some(col) => { match col.get((c+1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r) as usize) {
                        Some(col) => { match col.get((c-1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r) as usize) {
                        Some(col) => { match col.get((c+1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r+1) as usize) {
                        Some(col) => { match col.get((c-1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r+1) as usize) {
                        Some(col) => { match col.get((c) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }
                    match in_grid.get((r+1) as usize) {
                        Some(col) => { match col.get((c+1) as usize) {Some(n) => {if *n == 1 {neighbor+=1}} None => {}} }
                        None => { }
                    }

                    if neighbor == 3 {
                        out_grid[r as usize][c as usize] = 1;
                    }
                    else {
                        out_grid[r as usize][c as usize] = 0;
                    }
                }
                else {continue}

                c += 1;
            }
            r += 1;
        }

        self.field = out_grid;
    }
}