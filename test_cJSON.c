// 测试cJSON核心流程：解析→格式化输出→无格式输出→释放
#include <stdio.h>
#include "cJSON.h"

int main()
{
    //定义简单的JSON测试字符串
    const char *json_str = "{\"name\":\"Tom\",\"age\":18,\"grade\":90.5}";

    //调用cJSON_Parse解析JSON字符串（Day2学习）
    cJSON *root = cJSON_Parse(json_str);
    if (root == NULL) {
        printf("JSON解析失败！\n");
        return 1;
    }
    printf("=== JSON解析成功 ===\n\n");

    //调用cJSON_Print输出格式化字符串（Day5学习）
    char *fmt_str = cJSON_Print(root);
    printf("1. 格式化输出（好看，适合阅读）：\n%s\n\n", fmt_str);

    //调用cJSON_PrintUnformatted输出无格式字符串（Day6学习）
    char *unfmt_str = cJSON_PrintUnformatted(root);
    printf("2. 无格式输出（紧凑，适合传输）：\n%s\n\n", unfmt_str);

    //释放所有内存（Day4学习）
    cJSON_Delete(root);   // 释放解析后的cJSON结构体
    free(fmt_str);        // 释放格式化输出的字符串
    free(unfmt_str);      // 释放无格式输出的字符串

    printf("=== 测试完成，内存已全部释放 ===\n");
    return 0;
}
