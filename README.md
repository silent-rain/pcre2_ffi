# 项目文档

这是是一个使用基于 Pcre2 进行 fust ffi 绑定的正则使用项目。

## Pcre2 编译

手动进行编译静态库文件。
注意: 该步骤已经实现了自动化编译。

```shell
# 克隆 Pcre2 项目
git clone https://github.com/PhilipHazel/pcre2

# 进入项目
cd pcre2

# 创建一个构建目录并进入该目录
mkdir build && cd build

# 运行 CMake 来生成构建系统
cmake ..

# 用生成的构建系统编译项目
cmake --build .

# 创建静态库文件
ar rcs linpcre2posix.a CMakeFiles/pcre2-posix-static.dir/src/pcre2posix.c.o
```
