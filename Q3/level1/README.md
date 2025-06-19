# 24k rust shell

本项目完成的是 Q3 中的 level1 题目。

Dockerhub链接: [24kcsplus/24k-shell-app](https://hub.docker.com/r/24kcsplus/24k-shell-app)

## 使用方法

### 构建

#### 使用 Docker
```shell
git clone git@github.com:24kcsplus/Redrock-SRE-2025-Ops-Winter-Assessment.git
cd Redrock-SRE-2025-Ops-Winter-Assessment/Q3/level1
sudo docker build -t 24k-shell-app .
sudo docker run -it --rm 24k-shell-app
```

### 拉取镜像

#### 使用 Docker
```shell
sudo docker pull 24kcsplus/24k-shell-app:latest
sudo docker run -it --rm 24kcsplus/24k-shell-app:latest
```

## 已实现功能
| 功能 | 描述                                       |
| ---- |------------------------------------------|
|cd| 切换目录                                     |
|exit| 退出 shell                                 |
|其它内建命令| pwd、echo、mkdir 和 exit 等                  |
|管道符| 支持管道符（\|）进行命令连接                          |
|重定向符| 支持输出重定向（>）、输入重定向（<）和追加到文件（>>）            |
|历史记录| 支持上下切换命令历史记录（使用 up 和 down 键）             |
|快捷键| 支持 Ctrl+C 中断命令、Ctrl+L 清屏、Ctrl+U 清除光标左侧内容 |

## 未实现功能
| 功能     | 描述                                           |
|--------|----------------------------------------------|
| 通配符    | 不支持通配符（如 *、? 等）进行文件匹配和操作                     |
| 环境变量   | 不支持环境变量的设置和使用（如 $PATH、$HOME 等）               |
| 命令别名   | 不支持命令别名的定义和使用（如 alias 命令）                    |
| 命令补全   | 不支持命令和文件名的自动补全功能（如 Tab 键补全）                  |
| 脚本执行   | 不支持执行脚本文件（如 .sh 文件）                          |
| 部分内建命令 | 不支持部分内建命令（如 history、export 等，大多为需要改变内部状态的指令） |
| 其它快捷键  | 不支持其他快捷键（如 Ctrl+R 搜索历史命令、Ctrl+D 退出 shell 等）  |

## 参考资料

- [Rust 官方文档](https://doc.rust-lang.org/book/)
- [Rust 语言参考](https://doc.rust-lang.org/reference/)
- [【译】使用 Rust 构建你自己的 Shell](https://www.cnblogs.com/ishenghuo/p/12550142.html)
- [CSAPP Shell Lab README](http://csapp.cs.cmu.edu/3e/README-shlab)
- [CSAPP Shell Lab Writeup](http://csapp.cs.cmu.edu/3e/shlab.pdf)
