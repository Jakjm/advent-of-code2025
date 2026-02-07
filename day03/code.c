#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
int64_t problemOne(FILE *inputFile){
    #define bufferSize 4196
    char inputChars[bufferSize];
    int64_t total = 0;
    char *bufStart = &inputChars[0];
    while(fgets(bufStart,bufferSize,inputFile)){
        int left = -1;
        char c;
        int maxPair = 0;
        for(char *p = bufStart;(c = *p) >= '0' && c <= '9';++p)
        {
            int digit = (c - '0');
            if(left != -1)
            {
                int64_t newCandidate = left * 10 + digit;
                if(maxPair < newCandidate)
                    maxPair = newCandidate;
            }
            if(digit > left)
                left = digit;
            
        }
        total += maxPair;
    }
    return total;
}
int64_t problemTwo(FILE *inputFile){
    #define bufferSize 4196
    char inputChars[bufferSize];
    int64_t total = 0;
    char *bufStart = &inputChars[0];
    while(fgets(bufStart,bufferSize,inputFile)){
        int64_t maxes[12] = {-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1};
        char c;
        for(char *p = bufStart;(c = *p) >= '0' && c <= '9';++p)
        {
            int digit = (c - '0');
            for(int64_t* p = &maxes[11]; p >= &maxes[0]; --p){
                int64_t cur = *p;
                int64_t candidate = -1;
                int64_t prev;
                if(p == &maxes[0])
                    candidate = digit;
                else if((prev = *(int64_t*)(p - 1)) >= 0)
                    candidate = (prev * 10) + digit;

                if(candidate > cur){
                    *p = candidate;
                }
            }
        }
        if(maxes[11] > 0)
            total += maxes[11];
    }
    return total;
}
int main(int argc, char **argv){
    FILE *inputFile = fopen("input.txt", "r");
    int64_t result = problemTwo(inputFile);
    printf("%ld\n", result);
    fclose(inputFile);
}