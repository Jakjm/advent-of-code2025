#include <stdio.h>
#include <string.h>
#include <stdint.h>

#define maxLines 512
#define bufferSize 4096
// Read the input from the file into the array.
// The two lines of the array contains only '.'
// The first character of each line contains an extra '.'
int64_t readInput(FILE *inputFile, char inputChars[maxLines][bufferSize])
{
    char *curLine;
    int lineCount = -1;
    do
    {
        ++lineCount;
        curLine = &inputChars[2 + lineCount][0];
        curLine[0] = '.';
        curLine = fgets(&curLine[1], bufferSize - 1, inputFile);
    } while (curLine);

    for (int i = 0; i < bufferSize; ++i)
    {
        inputChars[0][i] = '.';
        inputChars[1][i] = '.';
        inputChars[2 + lineCount][i] = '.';
    }
    inputChars[0][bufferSize - 1] = '\0';
    inputChars[1][bufferSize - 1] = '\0';
    inputChars[2 + lineCount][bufferSize - 1] = '\0';
    return lineCount;
}
int64_t problemTwo(FILE *inputFile)
{
    char inputChars[maxLines][bufferSize];
    int64_t lineCount = readInput(inputFile,inputChars);
    int64_t total = 0;
    int newCount;
    do
    {
        newCount = 0;
        for(int lineCt = 0; lineCt <= lineCount; ++lineCt)
        {
            char *prevPrevLine = &inputChars[lineCt][1];
            char *prevLine = &inputChars[lineCt + 1][1];
            char *curLine = &inputChars[lineCt + 2][1];
            int lineEnd = 0;
            for (int i = 0; !lineEnd; ++i)
            {
                char c = prevLine[i];
                if (c == '@')
                {
                    int sqValue = (prevPrevLine[i - 1] == '@') +
                                (prevPrevLine[i] == '@') +
                                (prevPrevLine[i + 1] == '@');
                    sqValue += (prevLine[i - 1] == '@') + (prevLine[i + 1] == '@');
                    sqValue += (curLine[i - 1] == '@') + (curLine[i] == '@') +
                            (curLine[i + 1] == '@');
                    if (sqValue < 4)
                    {
                        ++newCount;
                        c = prevLine[i] = '.';
                    }
                }
                else
                    lineEnd = c != '.';
                putchar(c);
            }
        }
        total += newCount;
    }while(newCount > 0);
    return total;
}
int64_t problemOne(FILE *inputFile){
    char inputChars[3][bufferSize];
    int64_t total = 0;

    for (int i = 0; i < bufferSize; ++i)
    {
        inputChars[0][i] = '.';
        inputChars[1][i] = '.';
        inputChars[2][i] = '.';
    }

    char *prevPrevLine = &inputChars[2][1];

    int lineCount = 1;
    char *prevLine = fgets(&inputChars[0][1], bufferSize - 1, inputFile);
    while(prevLine)
    {
        char *curLine = fgets(&inputChars[lineCount % 3][1], bufferSize - 1, inputFile);
        int lineEnd = 0;
        int prev = 0;
        int cur = 0;
        for (int i = 0; !lineEnd; ++i)
        {
            char c = prevLine[i];
            if (c == '@')
            {
                int sqValue = (prevPrevLine[i - 1] == '@') +
                                (prevPrevLine[i] == '@') +
                                (prevPrevLine[i + 1] == '@');
                sqValue += (prevLine[i - 1] == '@') + (prevLine[i + 1] == '@');
                if(curLine)
                    sqValue += (curLine[i - 1] == '@') + (curLine[i] == '@') +
                                (curLine[i + 1] == '@');
                if (sqValue < 4)
                {
                    ++total;
                    c = 'x';
                }
            }
            else
                lineEnd = c != '.';
            putchar(c);
        }
        ++lineCount;
        prevPrevLine = prevLine;
        prevLine = curLine;
    }
    return total;
}
int64_t bonus(FILE *inputFile){

    char inputChars[bufferSize];
    int64_t total = 0;
    char *bufStart = &inputChars[0];
    while(fgets(bufStart,bufferSize,inputFile)){
        int left = 0; 
        char c;
        int count = 0;
        char *p;
        char *min4;
        char *min8;
        char *bufEnd = NULL;
        //LI: left = the number of rolls in the 8 positions preceeding p. 
        for(p = bufStart;;++p)
        {
            min4 = p - 4;
            if(!bufEnd)
            {
                c = *p;
                if(c == '@')
                    ++left;
                else if(c != '.')
                    bufEnd = p;
            }
        
            if(bufStart <= min4 && (!bufEnd || min4 < bufEnd))
            {
                c = *min4;
                if(c == '@' && left <= 4)
                {
                    *min4 = 'x';
                    ++count;
                }
            }
            else if(bufEnd)
                break;
            min8 = p - 8;
            if(bufStart <= min8)
            {
                c = *min8;
                if(c == '@')
                    --left;
            }
            total += count;
        }
        printf("%s",bufStart);
    }
    return total;
}
int main(int argc, char **argv){
    FILE *inputFile = fopen("input.txt", "r");
    int64_t result = problemTwo(inputFile);
    printf("%ld\n", result);
    fclose(inputFile);
}