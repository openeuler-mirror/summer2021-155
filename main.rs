use std::io;

use std::default::Default;

#[derive(Copy, Clone)]
struct Point
{
    name:   char,
    flag:   i32     //0 start  1 end  2 normal 
}

impl Default for Point {
    fn default() -> Point {
        Point {
            name: '-',
            flag: 0
        }
    }
}

impl Point {
    fn run1 (&self, k: usize) -> () {
        println!("定点 {}: {}:{}", k, self.name, self.flag);
    }

}

#[derive(Copy, Clone)]
struct Line
{
    start:  Point,
    end:    Point,
    len:    i32
}

impl Default for Line {
    fn default() -> Line {
        Line {
            start:  Point::default(),
            end:    Point::default(),
            len:    0
        }
    }
}

struct Dag
{
    point: [Point; 50],
    line: [Line; 100],
    line_count: i32,
    point_count: i32
}

impl Default for Dag {
    fn default() -> Dag {
        Dag {
            point: [Point::default(); 50],
            line: [Line::default(); 100],
            line_count: 0,
            point_count: 0
        }
    }
}

impl Dag {
    fn determine_the_node_type (&mut self) -> () {
        let mut start: [i32; 255] = [0; 255];
        let mut end: [i32; 255] = [0; 255];

        let mut i: i32 = 0;
        while i < self.line_count {
            start[self.line[i as usize].start.name as usize] = 2;
            end[self.line[i as usize].end.name as usize] = 2;
            i += 1;
        }
        
        i = 0;
        while i < self.point_count {
            if start[self.point[i as usize].name as usize] == 2 && end[self.point[i as usize].name as usize] == 2 {
                self.point[i as usize].flag = 2; //normal点
            }
            else if start[self.point[i as usize].name as usize] == 0 && end[self.point[i as usize].name as usize] == 2 {
                self.point[i as usize].flag = 1; //结束点
            }
            else if start[self.point[i as usize].name as usize] == 2 && end[self.point[i as usize].name as usize] == 0 {
                self.point[i as usize].flag = 0; //开始点
            }

            i += 1;
        }
    }

    fn check (&self) -> (){
        if self.line_count == 0 {
            println!("不存在边\n");
        }
        if self.point_count == 0 {
            println!("不存在节点\n");
        }
    }

    fn run (&self) -> () {
        let mut i: i32 = 0;

        if self.line_count != 0 {
            while i < self.line_count {
                println!("{}->{}:{}\n", self.line[i as usize].start.name, self.line[i as usize].end.name, self.line[i as usize].len);
                i += 1;
            }
            return;
        }
        if self.point_count != 0 {
            i = 0;
            while i < self.point_count {
                self.point[i as usize].run1((i + 1) as usize);
                i += 1;
            }
            return;
        }
        println!("不存在图！");
    }

    fn get_start_point (&self) -> () {
        let mut i: i32 = 0;

        while i < self.point_count {
            if self.point[i as usize].flag == 0 {
                println!("开始节点：{} {}\n", self.point[i as usize].name, self.point[i as usize].flag);
            }
            i += 1;
        }
    }

    fn get_end_point (&self) -> () {
        let mut i: i32 = 0;

        while i < self.point_count {
            if self.point[i as usize].flag == 1 {
                println!("结束节点：{} {}\n", self.point[i as usize].name, self.point[i as usize].flag);
            }
            i += 1;
        }
    }

}

fn get_char() -> char {
    let mut tmp_val: String = String::new();
    let mut value: char = '-';
    io::stdin().read_line(&mut tmp_val).expect("Failed to read line");
    let trimmed = tmp_val.trim();
    match trimmed.parse::<char>() {
        Ok(_i) => value = _i,
        Err(..) => { println!("this was not an char: {}", trimmed) },
    }
    value
}

fn get_i32() -> i32 {
    let mut tmp_val: String = String::new();
    let mut value: i32 = 0;
    io::stdin().read_line(&mut tmp_val).expect("Failed to read line");
    let trimmed = tmp_val.trim();
    match trimmed.parse::<i32>() {
        Ok(_i) => value = _i,
        Err(..) => { println!("this was not an integer: {}", trimmed) },
    }
    value
}

fn main() {
    let mut dag = &mut Dag::default();

    println!("输入定点数:");
    dag.point_count = get_i32();

    let mut i: i32 = 0;
    while i < dag.point_count {
        println!("第 {} 个定点：", i + 1);
        dag.point[i as usize].name = get_char();
        dag.point[i as usize].flag = 3;      // 默认设置为单独的孤点，没有任何线相连；
        i += 1;
    }

    println!("输入边数: ");
    dag.line_count = get_i32();

    let mut a: char;
    let mut b: char;
    let mut flag: i32;
    let mut flag1: i32;
    
    i = 0;
    while i < dag.line_count {
        println!("第 {} 条边和权重 [例如输入 a b c，即表示 a->b，权重为 c]:", i + 1);

        a = get_char();
        b = get_char();

        flag = 0;
        flag1 = 0;

        let mut l: i32 = 0;
        let mut j: i32 = 0;

        while j < dag.point_count {
            if dag.point[j as usize].name == a {
                flag = j;
                l += 1;
            }
            if dag.point[j as usize].name == b {
                flag1 = j;
                l += 1;
            }
            j += 1;
        }

        if l != 2 {
            if i > 0 { i -= 1; }
            println!("输入节点不存在!");
            continue;
        }
        dag.line[i as usize].start = dag.point[flag as usize];
        dag.line[i as usize].end = dag.point[flag1 as usize];
        dag.line[i as usize].len = get_i32();
        i += 1;
    }
    
    dag.determine_the_node_type();

    dag.check();    // 检查定点是否有边或者定点；
    dag.run();      // 打印图；


    dag.get_start_point();    // 图的开始节点；
    dag.get_end_point();      // 图的结束节点；

    return;
}