use std::env;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io;
struct Pod {
    people: Vec<String>,
}
struct Arrange {
    pods: Vec<Pod>,
    mod_n: i32,
}
fn main() {
    let mut relations = HashMap::new();
    let mut class: Vec<String> = Vec::new();
    let args = env::args().collect::<Vec<String>>();
    if &args.len() < &2 {
        println!("You did not specify a source file.");
    } else {
        let mut f = File::open(String::from(args[1].as_ref())).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Couldn't read source file");
        let res = int_f(contents);
        let total = res.1;
        let ti = total.iter();
        for r in ti {
            for i in r.pods.iter() {
                for p in i.people.iter() {
                    if !class.contains(p) {
                        class.push(String::from(p.as_ref()));
                    }
                    let re = relations.entry(String::from(p.as_ref())).or_insert(
                        HashMap::new(),
                        );
                    for d in i.people.iter() {
                        if d != p {
                            let ref_i = re.entry(String::from(d.as_ref())).or_insert(0);
                            *ref_i += r.mod_n;
                        }
                    }
                }
            }
        }
        relations = custom_rels(relations);
        let mut res = create_assign(relations, class);
        let mut i = 0;
        for r in res.pods {
            println!("Pod {}: {:?}", i, r.people);
            i += 1;
        }
    }
}
fn custom_rels(
    rels: HashMap<String, HashMap<String, i32>>,
    ) -> HashMap<String, HashMap<String, i32>> {
    let mut nrels = rels.clone();
    println!("Would you like to create a custom association?");
    print!("(y/n)>");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    if buf.trim() == "y" || buf.trim() == "yes" {
        let mut con = true;
        while con {
            print!("First person: >");
            io::stdout().flush().unwrap();

            let mut first = String::new();
            io::stdin().read_line(&mut first).unwrap();
            print!("Second person: >");
            io::stdout().flush().unwrap();

            let mut sec = String::new();
            io::stdin().read_line(&mut sec).unwrap();
            print!("Amount to affect the relations by >");
            io::stdout().flush().unwrap();

            let mut amount = String::new();
            io::stdin().read_line(&mut amount).unwrap();
            {
                let mut got = nrels.get_mut(first.trim());

                if got.is_some() {
                    *got.unwrap().entry(String::from(sec.trim())).or_insert(0) +=
                        amount.trim().parse::<i32>().unwrap();
                    println!("Association link 1 edited!");

                } else {
                    println!("{} is not a valid person!", first.trim());
                }
            }
            {
                let mut ru = nrels.get_mut(sec.trim());
                if ru.is_some() {
                    *ru.unwrap().entry(String::from(first.trim())).or_insert(0) += amount.trim().parse::<i32>().unwrap();
                    println!("Association link 2 edited!");
                } else {
                    println!("{} is not a valid person!", sec.trim());
                }
            }
            println!("Would you like to edit another association?");
            print!("(y/n");
            io::stdout().flush().unwrap();
            let mut chos = String::new();
            io::stdin().read_line(&mut chos).unwrap();
            if chos.trim() == "y" || chos.trim() == "yes" {
                //Loop continues
            } else if chos.trim() == "n" || chos.trim() == "no" {
                con = false;
            }

        }
        return nrels;
    } else if buf.trim() == "n" || buf.trim() == "no" {
        return nrels;
    } else {
        println!("Answer unrecognized");
        return custom_rels(nrels);
    }
}
fn create_assign(rels: HashMap<String, HashMap<String, i32>>, class: Vec<String>) -> Arrange {
    let mut pods: Vec<Pod> = Vec::new();
    let mut self_c = class.clone();
    let i = 0;
    while 0 < self_c.len() {
        let p = String::from(self_c[i].as_ref());
        let cur = rels.get(&p).unwrap();
        self_c.sort_unstable_by(|a, b| {
            let ag = cur.get(a);
            if ag.is_some() {
                let bg = cur.get(b);
                if bg.is_some() {
                    return bg.unwrap().cmp(ag.unwrap());
                } else {
                    if ag.unwrap() > &0 {
                        return Ordering::Less;
                    } else {
                        return Ordering::Greater;
                    }
                }
            } else {
                let bg = cur.get(b);
                if bg.is_some() {
                    if bg.unwrap() > &0 {
                        return Ordering::Greater;
                    } else {
                        return Ordering::Less;
                    }
                }
            }
            return Ordering::Equal;
        });
        self_c.remove(i);
        let fi = self_c.pop();
        let mut fi_d:String;
        if fi.is_none() {
            fi_d = String::from("");
        } else {
            fi_d = fi.unwrap();
        }
        let sec = self_c.pop();
        let mut sec_d : String;
        if sec.is_none() {
            sec_d = String::from("");
        } else {
            sec_d = sec.unwrap();
        }
        if p == fi_d {
            fi_d = String::from("");
        }
        if p == sec_d {
            sec_d = String::from("");
        }
        pods.push(Pod { people: vec![p, fi_d, sec_d] });

    }
    Arrange {
        pods: pods,
        mod_n: 0,
    }
}
fn int_f(file: String) -> (i32, Vec<Arrange>) {
    let mut total: Vec<Arrange> = Vec::new();
    let sp = file.split("|").collect::<Vec<&str>>();
    let pre = sp[0];
    let sec = sp[1];
    let p_size = pre.trim().parse().unwrap();
    let sp2 = sec.split("-");
    for s in sp2 {
        let mut p = 0;
        let mut pds: Vec<Pod> = Vec::new();
        let mut cur: Vec<String> = Vec::new();
        for l in s.trim().lines() {
            p = p + 1;
            cur.push(String::from(l));
            if p_size <= p {
                p = 0;
                pds.push(Pod { people: cur.clone() });
                cur = Vec::new();
            }
        }
        total.push(Arrange {
            pods: pds,
            mod_n: p + 1,
        });
    }
    (p_size, total)
}
