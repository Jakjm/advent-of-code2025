#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <stdint.h>
//Return floor(log_10(x)), where x is an integer.
int log_base10(int64_t x)
{
    int result = 0;
    int64_t cur = 1;
    while(cur <= x)
    {
        cur *= 10;
        result++;
    }
    --result;
    return result;
}
//Return 10^x, where x is an integer.
int64_t pow_base10(int x){
    int64_t result = 1;
    for(int i = 0;i < x;++i){
        result *= 10;
    }
    return result;
}
// int64_t bad_ceil_sqrt(int x){
//     int i = 0;
//     while(i*i < x){
//         ++i;
//     }
//     return i;
// }
int64_t problemOne(FILE *inputFile){
    int64_t rangeStart, rangeEnd;
    int64_t sumInvalids = 0;
    while(fscanf(inputFile, "%ld-%ld ", &rangeStart, &rangeEnd) == 2)
    {
        int64_t cur = rangeStart;
        int numDigits;
        //printf("%ld   %ld\n", rangeStart, rangeEnd);
        while(cur <= rangeEnd){
            numDigits = log_base10(cur) + 1;
            if(numDigits % 2 == 1){
                //printf("Moving up %ld ", cur);
                cur = pow_base10(numDigits);
                //printf("%ld\n",cur);
                continue;
            }
            int x = pow_base10(numDigits / 2);
            int firstHalf = (cur / x);
            int64_t candidateInvalid = firstHalf;
            candidateInvalid *= x;
            candidateInvalid += firstHalf;
            if(rangeStart <= candidateInvalid && candidateInvalid <= rangeEnd)
            {
                sumInvalids += candidateInvalid;
                //printf("Cur %ld Candidate %ld\n",cur, candidateInvalid);
            }

            cur = (firstHalf + 1);
            cur *= x;
            cur += (firstHalf + 1);
        }
    }
    return sumInvalids;
}
int64_t problemTwo(FILE *inputFile)
{
    int64_t rangeStart, rangeEnd;
    int64_t sumInvalids = 0;
    while(fscanf(inputFile, "%ld-%ld ", &rangeStart, &rangeEnd) == 2)
    {
        printf("Range: %ld %ld\n\t",rangeStart, rangeEnd);
        for(int64_t cur = rangeStart; cur <= rangeEnd; ++cur){
            int numDigits = log_base10(cur) + 1;
            //int sqrt = bad_ceil_sqrt(numDigits);
            int chunksMatch = 0;
            for(int numChunks = 1; numChunks <= numDigits;++numChunks)
            {
                if(numDigits % numChunks == 0 && numChunks > 1)
                {
                    int chunkSize = numDigits / numChunks;
                    //printf("%ld trying %d \n",cur,chunkSize);
                    int x = pow_base10(chunkSize);
                    chunksMatch = 1;
                    int64_t curCopy = cur;
                    int64_t firstChunk = curCopy % x;
                    //printf("%ld", firstChunk);
                    for(int i = 1;chunksMatch && (i < numChunks);++i){
                        curCopy /= x;
                        int64_t curChunk = curCopy % x;
                        //printf(" %ld",curChunk);
                        chunksMatch = (curChunk == firstChunk);
                    }
                    //putchar('\n');
                    if(chunksMatch)
                        break;
                }
            }
            if(chunksMatch)
            {    
                printf("%ld ", cur);
                sumInvalids += cur;
            }
            //sleep(2);
        }
        putchar('\n');
    }
    return sumInvalids;
}
int main(int argc, char **argv){
    FILE *inputFile = fopen("input.txt", "r");
    int64_t sumInvalids = problemTwo(inputFile);
    fclose(inputFile);
    printf("%ld\n", sumInvalids);
}