#include<stdio.h>
#include<malloc.h>


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
int main()
{
    struct DAG dag;
    dag.lineCount = dag.pointCount = 0;
    int i, j;
    printf("输入定点数:");
    scanf("%d", &dag.pointCount);
    for(i = 0; i<dag.pointCount; i++)
    {
        printf("第%d个定点：", i+1);
        getchar();
        scanf("%c", &dag.point[i].name);

        dag.point[i].flag = 3; ///默认设置为单独的孤点，没有任何线相连
        dag.point[i].run1 = run1;
    }
    printf("输入边数: ");
    char a, b;
    int flag, flag1;

    scanf("%d", &dag.lineCount);
    if(dag.lineCount!=0)
        getchar();
    for(i = 0; i<dag.lineCount; i++)
    {
        printf("第%d条边和权重[例如输入a b c，即表示a->b，权重为c]:", i+1);
        scanf("%c %c", &a, &b);
        flag = flag1 = 0;
        int l = 0;
        for(j = 0; j<dag.pointCount; j++)
        {
            if(dag.point[j].name == a)
            {
                flag = j;
                l++;
            }
            if(dag.point[j].name == b)
            {
                flag1 = j;
                l++;
            }
        }
        if(l != 2)
        {
            i -= 1;
            printf("输入节点不存在!");
            continue;
        }
        dag.line[i].start = dag.point[flag];
        dag.line[i].end = dag.point[flag1];
        scanf("%d", &dag.line[i].len);
        getchar();
    }
    Determine_The_Node_Type(&dag);///不能删，这是判断节点的类型的start\end\normal点

    dag.check = check;
    dag.run = run;
    dag.getStartPoint = getStartPoint;  ///这个结构体有多少个函数，就要在这个有多少个结构体内，函数指针指向函数的声明。
    dag.getEndPoint = getEndPoint;

    dag.check(dag);///检查定点是否有边或者定点
    dag.run(dag);///打印图

    dag.getStartPoint(dag);///图的开始节点
    dag.getEndPoint(dag);///图的结束节点


    return 0;
}
