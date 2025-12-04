#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <deque>
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
        std::deque<int> maxes;
        int left = -1;
        char c;
        for(char *p = bufStart;(c = *p) >= '0' && c <= '9';++p)
        {
            int digit = (c - '0');
            if(maxes.size() >= 11)
            {
                int64_t newCandidate = maxes[10] + digit;
                if(maxes[11] < newCandidate)
                    maxes[11] = newCandidate;
            }
            if(digit > left)
                left = digit;
            
        }
        total += maxes[11];
    }
    return total;
}
int main(int argc, char **argv){
    FILE *inputFile = fopen("input.txt", "r");
    int64_t result = problemOne(inputFile);
    printf("%ld\n", result);
    fclose(inputFile);
}