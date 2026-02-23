# cJSON项目环境搭建问题记录
## 1. WSL安装相关问题
- 问题1：执行`wsl--install --web-download`提示“不是内部或外部命令”；
  解决：命令参数间需加空格，正确格式为`wsl --install --web-download`；
- 问题2：执行`wsl --set-default-version 2`提示“需更新WSL组件”；
  解决：先执行`wsl --update`更新WSL内核，再重新设置默认版本；

## 2. Git克隆相关问题
- 问题：在Windows挂载目录（/mnt/c/Users/张紫昂）克隆cJSON提示“chmod failed: Operation not permitted”；
  解决：切换到WSL原生目录（/home/idawy）克隆，避免Windows文件系统权限冲突；

## 3. 编译相关问题
- 问题：首次编译前未安装gcc，提示“gcc: command not found”；
  解决：执行`sudo apt install gcc git make -y`安装编译工具。
