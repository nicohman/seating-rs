use std::env;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
struct Pod {
    people: Vec<String>,
}
struct Arrange {
    pods: Vec<Pod>,
    modN: i32,
}
fn main() {
    let mut relations = HashMap::new();
    let mut class: Vec<String> = Vec::new();
    let args = env::args().collect::<Vec<String>>();
    let command: &str;
    if &args.len() < &2 {
        println!("You did not specify a source file.");
    } else {
        let mut f = File::open(String::from(args[1].as_ref())).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents);
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
                            let refI = re.entry(String::from(d.as_ref())).or_insert(0);
                            *refI += r.modN;
                        }
                    }
                }
            }
        }
        let mut res = create_assign(relations, class);
        for r in res.pods {
            println!("Pod: {:?}", r.people);
        }
    }
}
fn create_assign(rels: HashMap<String, HashMap<String, i32>>, class: Vec<String>) -> Arrange {
    let mut pods: Vec<Pod> = Vec::new();
    let mut self_c = class.clone();
    let i = 0;
    while 0 < self_c.len() {
        let p = String::from(self_c[i].as_ref());
        let mut cur = rels.get(&p).unwrap();
        self_c.sort_by(|a, b| {
            let ag = cur.get(a);
            if ag.is_some() {
                let bg = cur.get(b);
                if bg.is_some() {
                    return ag.unwrap().cmp(bg.unwrap());
                }
            }
            return Ordering::Equal;
        });
        self_c.remove(i);
        let fi = self_c.pop();
        let mut fi_d = String::new();
        if fi.is_none() {
            fi_d = String::from("");
        } else {
            fi_d = fi.unwrap();
        }
        let sec = self_c.pop();
        let mut sec_d = String::new();
        if sec.is_none() {
            sec_d = String::from("");
        } else {
            sec_d = sec.unwrap();
        }
        pods.push(Pod {
            people: vec![p, fi_d, sec_d],
        });
        
    }
    Arrange {
        pods: pods,
        modN: 0,
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
            modN: p + 1,
        });
    }
    (p_size, total)
}
