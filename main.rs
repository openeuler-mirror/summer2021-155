use std::string::String;
use libc::{c_int, c_void};

struct Point {
    Pointname: String,
    flag: u64,    //0 start 1 end 2 noprmal
    run: String,
}

struct Line {
    Pointstart: u64,
    Pointend: u64,
    len: u64,
}

struct DAG {
    Point: u64,
    Line: u64,
    lineCount: u64,
    pointCount: u64,
    check: String,
}

void run1(struct Point point, int k)
{
    println!("定点%d: %c:%d\n", k, point.name, point.flag);
}

void Determine_The_Node_Type(struct DAG *dag)
{
    let int value = i;
    let int start[255] = {0};
    let int end[255] = {0};

    for(i = 0; i<dag->lineCount; i++)
    {
        start[dag->line[i].start.name] = 2;
        end[dag->line[i].end.name] = 2;
    }
    for(i = 0; i<dag->pointCount; i++)
    {
        if(start[dag->point[i].name] == 2 && end[dag->point[i].name]==2)
        {
            dag->point[i].flag = 2; ///normal点
        }
        else if(start[dag->point[i].name] == 0 && end[dag->point[i].name]==2)
        {
            dag->point[i].flag = 1; ///结束点
        }
        else if(start[dag->point[i].name] == 2 && end[dag->point[i].name]==0)
        {
            dag->point[i].flag = 0; ///开始点
        }
    }
}

void check(struct DAG dag)
{
    if(dag.lineCount == 0)
    {
        println!("不存在边\n");
    }
    if(dag.pointCount == 0)
    {
        println!("不存在节点\n");
    }
}

fn main() {
    let flag = Point{Pointname: start, end, normal, flag: 0, 1, 2, run: String};
    let Line = Line{Pointstart: a, Pointend: e, len}
}