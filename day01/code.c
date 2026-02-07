#include <stdio.h>
#include <stdint.h>
//#include <stdlib.h> 
#include <unistd.h>

int countZeros(FILE *file){
    int val;
    char dir;
    int64_t cur = 50;
    uint64_t numZeroes = 0;
    //printf("Starting 
    //at %ld\n", cur);
    while(fscanf(file,"%c%d\n",&dir,&val) == 2)
    {
        if(dir == 'R')
            cur += val;
        else  
            cur -= val;

        if(cur % 100 == 0){
            ++numZeroes;
        }
    }
    return numZeroes;
}
int countRevolutions(FILE *file){
    int val;
    char dir;
    int64_t cur = 50;
    uint64_t numRevs = 0;
    //LI: 0 <= cur <= 99
    while(fscanf(file,"%c%d\n",&dir,&val) == 2)
    {
        int old_num_revs = numRevs;
        int oldPos = cur;
        if(dir == 'R')
        {    
            cur += val;
            numRevs += (cur / 100);
        }
        else  
        {
            cur -= val;
            if(cur <= 0)
            numRevs += (oldPos != 0) + (-cur / 100);
        }
        //printf("Started %d. Turning %c by %d, leading to %ld, total revs %ld\n",oldPos,dir,val,cur,numRevs);
        //sleep(2);
        cur = cur % 100;
        cur = cur < 0 ? cur + 100 : cur;
    }
    return numRevs;
}
int main(int argc, char**argv){
    FILE *file = fopen("input.txt", "r");
    if(!file){
        return 1;
    }
    int numRevolutions = countRevolutions(file);
    fclose(file);
    printf("%d\n", numRevolutions);
}