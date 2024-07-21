#include <stdio.h>
#include <string.h>

int main(int argc, char *argv[]){
    char command[100]="";
    int command_flag = 0; // 使用int类型代替bool
    char filename[100] = "";
    int filename_flag = 0; // 使用int类型代替bool
    int i, j;

    if(argc > 1){
        for(i = 1; i < argc; i++){
            char temp[100]="";
            int temp_filename = 0; // 用于记录临时字符串的长度
            int temp_command=strlen(command);

            for(j = 0; argv[i][j] != '\0'; j++){
                if(argv[i][j] == '-'){
                    command_flag = 1;
                    continue;
                }
                else if(argv[i][j] == '.'){
                    filename_flag = 1;
                } else if(command_flag) {
                    if(strlen(command)+1>=sizeof(command)){
                        printf("Error:Command too long.\n");
                        return 1;
                    }
                    command[temp_command++] = argv[i][j]; // 正确地添加字符到command字符串
                }
                temp[temp_filename++] = argv[i][j]; // 正确地添加字符到temp字符串
            }
            temp[temp_filename] = '\0'; // 确保字符串以null字符结尾

            if(filename_flag){
                strcpy(filename, temp);
                filename_flag = 0;
            }
        }
    } else {
        printf("No arguments!\n");
    }
    printf("Command: %s\n", command);
    printf("Filename: %s\n", filename);
    return 0;
}