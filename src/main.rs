use rtree::link::Node;

#[derive(Clone)]
struct MetaData{
    pid :u32,
    ppid :u32,
    raw_info :String,
}

fn main() {
    //println!("Hello, world!");

    let mut pspools = Vec::new();
    let psinfo = vec![
        "root             1     0 0 19:10 ?        00:00:24 init second_stage",
        "root             2     0 0 19:10 ?        00:00:00 [kthreadd]",
        "root             4     2 0 19:10 ?        00:00:00 [kworker/0:0H]",
        "system        3878     1 1 19:23 ?        00:00:30 ylog",
        "root             6     2 0 19:10 ?        00:00:00 [mm_percpu_wq]",
        "system       14066  4068 4 42:57 ?        00:00:00 sleep 60s",
        "root          3895     1 0 19:23 ?        00:00:02 zygote64",
        "system        4906  3895 1 20:08 ?        00:00:59 system_server",
        "radio         6939  3895 0 21:40 ?        00:00:02 com.android.phone",
        "system        4068  3878 0 19:24 ?        00:00:07 sh /system/bin/ylogdebug.sh",
    ];

    for ps in psinfo{
        //println!("{}",ps);
        let items = ps.split_whitespace();
        //println!("{}",items[2]);
        let mut md = MetaData{
            pid :1,
            ppid:0,
            raw_info:ps.to_string(),
        };

        let mut index = 0;
        for i in items {
            //println!("....{}",i);
            if index == 1{
                //pid
                md.pid = i.parse::<u32>().unwrap();
            } else if index == 2{
                //ppid
                md.ppid = i.parse::<u32>().unwrap();
            }
            
            index+=1;
        }

        pspools.push(md);
    }

    let mt = find_pid_node(&pspools,1).expect("not find");
    let mut root = Node::new(mt);
    //let children = find_ppid_node(&pspools,1);
    gentree(&pspools,&mut root);
    println!("==========================");
    dump(&root,"|".to_string());
}

fn find_pid_node(pool:&Vec<MetaData>,pid: u32) -> Option<MetaData> {
    for i in pool {
        if i.pid == pid {
            let item = MetaData{
                pid: i.pid,
                ppid: i.ppid,
                raw_info: i.raw_info.clone(),
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
                raw_info: i.raw_info.clone(),
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

fn dump(root :&Node<MetaData>,prefix: String){
    println!("{}--↳{}",prefix,root.value.raw_info);
    let len = root.children.len();
    let index = 0;
    for child in &root.children{
        if index < len -1{
            dump(&child,String::from(prefix.clone()+"  |"))
        } else {
            let tmpstr = prefix.clone();
            dump(&child,String::from(tmpstr[0..tmpstr.len()-2].to_string()+"   |"));
        }
    }
}
