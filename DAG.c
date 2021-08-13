struct Point
{
    char name;
    int flag;///0 start  1 end  2 normal 3孤点
    void (*run1)(struct Point point, int k);
};

struct Line
{
    struct Point start;
    struct Point end;
    int len;
};

struct DAG
{
    struct Point point[50];
    struct Line line[100];
    int lineCount;
    int pointCount;

    void (*getStartPoint)(struct DAG dag);
    void (*getEndPoint)(struct DAG dag);
    void (*run)(struct DAG dag);
    void (*check)();
};

void getStartPoint(struct DAG dag)
{
    int i;
    for(i = 0; i<dag.pointCount; i++){

        if(dag.point[i].flag == 0){
            printf("开始节点：%c %d\n", dag.point[i].name, dag.point[i].flag);
        }
    }
}
void getEndPoint(struct DAG dag)
{
    int i;
    for(i = 0; i<dag.pointCount; i++){
        if(dag.point[i].flag == 1){
            printf("结束节点：%c %d\n", dag.point[i].name, dag.point[i].flag);
        }
    }
}

void run(struct DAG dag)
{
    int i;
    if(dag.lineCount != 0)
    {

        for(i = 0; i<dag.lineCount; i++)
        {
            printf("%c->%c:%d\n", dag.line[i].start.name, dag.line[i].end.name, dag.line[i].len);
        }
        return ;
    }
    if(dag.pointCount != 0)
    {
        for(i = 0; i<dag.pointCount; i++)
        {
            dag.point[i].run1(dag.point[i], i+1);
        }
        return ;
    }
    printf("不存在图！");
}
void check(struct DAG dag)
{
    if(dag.lineCount == 0)
    {
        printf("不存在边\n");
    }
    if(dag.pointCount == 0)
    {
        printf("不存在节点\n");
    }
}

void run1(struct Point point, int k)
{
    printf("定点%d: %c:%d\n", k, point.name, point.flag);
}

void Determine_The_Node_Type(struct DAG *dag)
{
    int i;
    int start[255] = {0};
    int end[255] = {0};

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
