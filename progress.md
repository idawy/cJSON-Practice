# cJSON项目进度记录（STEP1）
- 今日任务：
  1. 完成cJSON核心结构体（struct cJSON）的深度注释，理解双向链表+树形结构的设计逻辑；
  2. 解决“找不到parse object函数”的问题,定位cJSON解析总入口函数cJSON_Parse；
  3. 给cJSON_Parse函数添加核心流程注释，明确JSON解析的整体逻辑；
  4. struct cJSON通过next/prev（双向链表）+ child（树形）实现JSON嵌套结构；
  5. cJSON_Parse是解析总入口，会根据JSON首字符调用不同类型的解析函数；
## Day2（找到并注释解析入口函数）
  1. 用搜索命令找到cJSON_Parse函数的实现代码，这是解析JSON字符串的入口；
  2. 给这个函数加了简单注释，理解了它的作用：接收JSON字符串，返回解析结果；
  3. 发现这个函数内部会调用另一个解析函数，是简化版的使用方式；
  4. 学会了用nano跳行号、用grep搜索代码的小技巧。
