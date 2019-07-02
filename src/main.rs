#![allow(dead_code)]
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let fname: Vec<_> = env::args().collect();
    if fname.len() == 1 {
        std::process::exit(0);
    }
    let mut s: String = "./tests/".to_string();
    s.push_str(&fname[1]);
    let path = Path::new(&s);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let v: Vec<_> = contents.chars().collect();
    let mut vec = vec![];
    for c in &v {
        if *c == '\n' || *c == '\r' {
            break;
        }
        vec.push(c);
    }
    let row: usize = contents[0..1].trim().parse().expect("not a number");
    let col: usize = contents[2..3].trim().parse().expect("not a number");
    let mut data = vec![vec!['.'; col]; row];
    let mut start = 5;
    while start < v.len() {
        println!("l= {:?}", start);
        let mut v1 = vec![];
        for c in &v[start..] {
            if *c == '\r' {
                break;
            }
            v1.push(c);
        }
        start = start + v1.len() + 2;
        println!("{:?}", v1);
        let sym = *v1[0];
        let rownum;
        let colnum;
        let dir;
        let mut number;

        if *v1[2] != '-' && *v1[4] != '-' {
            rownum = *v1[2] as usize;
            colnum = *v1[4] as usize;
            dir = *v1[6];
            number = *v1[8] as usize;
        } else if *v1[2] != '-' {
            rownum = *v1[2] as usize;
            colnum = 48;
            dir = *v1[7];
            number = *v1[9] as usize;
        } else if *v1[4] != '-' {
            rownum = 48;
            colnum = *v1[5] as usize;
            dir = *v1[7];
            number = *v1[9] as usize;
        } else {
            rownum = 48;
            colnum = 48;
            dir = *v1[8];
            number = *v1[10] as usize;
        }

        //println!("sym {:?},rownum {:?},colnum {:?},dir {:?},number {:?} ",sym,rownum,colnum,dir,number);

        if dir == 'h' {
            if number - 48 > col {
                number = col + 48;
            }
            for i in 0..number - 48 {
                data[rownum - 48][i + colnum - 48] = sym;
            }
        } else {
            if number - 48 > row {
                number = row + 48;
            }
            for j in 0..number - 48 {
                data[rownum - 48 + j][colnum - 48] = sym;
            }
        }
        for i in data.iter() {
            println!("{:?}", i);
        }
    }

    let mut file = File::create("./tests/test.out")?;
    for i in data.iter() {
        writeln!(file,"{:?}",i)?;
    }


    Ok(())
    //
}
