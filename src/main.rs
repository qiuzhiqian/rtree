use rtree::link::Node;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Clone,Debug)]
struct MetaData{
    pid :u32,
    ppid :u32,
    cmdline :String,
}

fn main() {
    let cmd = clap::Command::new("rtree")
        .bin_name("rtree")
        .author("xiamengliang@gmail.com")
        .version("1.0.0")
        .about("This is a tool for format ps to pstree!!!")
        .args([
            clap::Arg::new("file")
                .long("file")
                .short('f')
                .action(clap::ArgAction::Append)
                .value_parser(clap::value_parser!(String)),
                
        ]);
    let matches = cmd.get_matches();

    let psinfo = if let Some(file) = matches.get_one::<String>("file") {
        let file_path = PathBuf::from(file);
        read_file_lines(file_path.to_str().expect("The path is not valid!!!"))
    } else {
        let mut psinfo = Vec::new();
        let lines = std::io::stdin().lines();
        for line in lines {
            if let Ok(line_str) = line {
                psinfo.push(line_str);
            }
        }
        psinfo
    };

    let pid_index = 1;
    let ppid_index = 2;
    let cmd_index = 7;

    let ps_pools = gen_ps_pools(psinfo,pid_index,ppid_index,cmd_index);

    let mt = find_pid_node(&ps_pools,1).expect("not find");
    let mut root = Node::new(mt);
    gentree(&ps_pools,&mut root);
    println!("===================================================");
    let root_prefixs = Vec::<String>::new();
    dump(&root,false,&root_prefixs,0,"");
}

fn read_file_lines(filepath:&str) -> Vec<String> {
    let file = std::fs::File::open(filepath).expect("file is not exist!!!");
    let mut lines = Vec::<String>::new();
    let file_lines = BufReader::new(file).lines();
    for line in file_lines {
        if let Ok(data) = line {
            lines.push(data);
        }
    }
    return lines;
}

fn gen_ps_pools(psinfo:Vec<String>,pid_index:u32,ppid_index:u32,cmd_index:u32) -> Vec<MetaData> {
    let mut pspools = Vec::new();
    for ps in psinfo{
        let items = ps.split_whitespace();
        let mut md = MetaData{
            pid :0,
            ppid:0,
            cmdline:String::new(),
        };

        let items_len = items.clone().count() as u32;
        if items_len < pid_index || items_len < ppid_index || items_len < cmd_index {
            continue;
        }

        let mut index = 0;
        for i in items {
            if index == pid_index{
                //pid
                if let Ok(x) = i.parse::<u32>() {
                    md.pid = x;
                } else {
                    continue;
                }
            } else if index == ppid_index{
                //ppid
                if let Ok(x) = i.parse::<u32>() {
                    md.ppid = x;
                } else {
                    continue;
                }
            } else if index >= cmd_index {
                if index > cmd_index{
                    md.cmdline.push_str(" ");
                }
                
                md.cmdline.push_str(i);

                if index == items_len -1 {
                    md.cmdline.push_str(&format!(" [{}]",md.pid));
                }
            }
            
            index+=1;
        }

        if md.pid !=0 {
            pspools.push(md);
        }
    }
    return pspools;
}

fn find_pid_node(pool:&Vec<MetaData>,pid: u32) -> Option<MetaData> {
    for i in pool {
        if i.pid == pid {
            let item = MetaData{
                pid: i.pid,
                ppid: i.ppid,
                cmdline: i.cmdline.clone(),
            };
            return Some(item);
        }
    }
    return None;
}

fn find_ppid_node(pool:&Vec<MetaData>,ppid: u32) -> Vec<MetaData> {
    let mut res = Vec::new();
    for i in pool {
        if i.ppid == ppid {
            let item = MetaData{
                pid: i.pid,
                ppid: i.ppid,
                cmdline: i.cmdline.clone(),
            };
            res.push(item);
        }
    }
    return res;
}

fn gentree(pool:&Vec<MetaData>,root :&mut Node<MetaData>){
    let res = find_ppid_node(pool,root.get_value().pid);
    for i in res {
        root.add_child(i);

        let len = root.children.len();
        gentree(pool,&mut root.children[len-1]);
    }
}

//"???"???"???"???"???"???"???"
fn dump(root :&Node<MetaData>,lastitem: bool,prefixs:&Vec<String>,deepin:u32,tag: &str){
    let mut full_str = String::new();
    for prefix in prefixs{
        full_str.push_str(&prefix);
    }
    full_str.push_str(tag);
    //println!("=={}",full_str);
    println!("{}{}",full_str,root.value.cmdline);
    let len = root.children.len();
    let mut index = 0;

    let new_prefixs = if !lastitem {
        let mut new_prefixs = prefixs.clone();
        if deepin > 0{
            new_prefixs.push("???".to_string());
        } else {
            new_prefixs.push(" ".to_string());
        }
        
        new_prefixs.push("  ".to_string());
        new_prefixs
    } else {
        let mut new_prefixs = prefixs.clone();
        if deepin > 0{
            new_prefixs.push("???".to_string());
        } else {
            new_prefixs.push(" ".to_string());
        }

        new_prefixs.pop().expect("is end.");
        new_prefixs.push("  ".to_string());
        new_prefixs.push("  ".to_string());
        new_prefixs
    };

    for child in &root.children{
        //println!("{} {}",index,len);
        if index < len -1{
            dump(&child,false,&new_prefixs,deepin+1,"??????");
        } else {
            dump(&child,true,&new_prefixs,deepin+1,"??????");
        }
        index+=1;
    }
}
