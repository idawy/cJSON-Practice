// 测试cJSON核心流程：解析→输出→释放
#include <stdio.h>
#include "cJSON.h"

int main()
{

    const char *json_str = "{\"name\":\"Tom\",\"age\":18}";

    // 2. 解析JSON字符串（调用Day2注释的cJSON_Parse）
    cJSON *root = cJSON_Parse(json_str);
    if (root == NULL) {
        printf("解析失败！\n");
        return 1;
    }

    // 3. 输出解析后的结果（调用Day5注释的cJSON_Print）
    char *print_str = cJSON_Print(root);
    printf("格式化输出JSON：\n%s\n", print_str);

    // 4. 释放内存（调用Day4注释的cJSON_Delete，避免内存泄漏）
    cJSON_Delete(root);
    free(print_str); // 释放Print返回的字符串内存

    printf("测试完成！\n");
    return 0;
}
