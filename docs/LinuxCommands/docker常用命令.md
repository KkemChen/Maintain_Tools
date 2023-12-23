---
layout: doc
title: Docker常用命令
---

# Docker常用命令

Docker是一个开放平台，用于开发、交付和运行应用程序。以下是一些常用的Docker命令，帮助你管理容器、镜像、网络和卷。

## 基本命令

- **`docker --version`**: 显示Docker版本信息。
- **`docker info`**: 显示Docker系统信息，包括镜像和容器数。

## 镜像管理

- **`docker images`**: 列出本地镜像。
- **`docker pull <image>`**: 下载一个镜像或仓库到本地。
- **`docker rmi <image>`**: 删除一个本地镜像。
- **`docker build -t <tag> .`**: 使用当前目录的Dockerfile创建镜像。

## 容器管理

- **`docker ps`**: 列出所有运行中的容器。
- **`docker ps -a`**: 列出所有容器，包括未运行的。
- **`docker run -d <image>`**: 后台运行容器。
- **`docker run -it <image>`**: 交互式运行容器，通常用于运行命令行应用。
- **`docker stop <container>`**: 停止一个运行中的容器。
- **`docker start <container>`**: 启动一个已停止的容器。
- **`docker restart <container>`**: 重启容器。
- **`docker rm <container>`**: 删除一个容器。

## 网络管理

- **`docker network ls`**: 列出所有网络。
- **`docker network create <name>`**: 创建一个新的网络。
- **`docker network rm <network>`**: 删除网络。

## 卷管理

- **`docker volume ls`**: 列出所有卷。
- **`docker volume create <name>`**: 创建一个新的卷。
- **`docker volume rm <volume>`**: 删除卷。

## 查看日志和进程

- **`docker logs <container>`**: 查看容器的日志。
- **`docker top <container>`**: 查看容器内运行的进程。

## 进入容器

- **`docker exec -it <container> bash`**: 进入运行中的容器并启动bash会话。

## 保存和加载镜像

- **`docker save <image> > image.tar`**: 保存镜像到文件。
- **`docker load < image.tar`**: 加载镜像文件。



## docker打包示例 
- 将容器打包为镜像

`docker commit <container id> nvidia_cuda11.8-cudnn8.5-trt8.5-driver535.98-ubuntu18.04:4.0`

- 将镜像保存为本地文件

`docker save -o nvidia_cuda11.8-cudnn8.5-trt8.5-driver535.98-ubuntu18.04_4.0.tar nvidia_cuda11.8-cudnn8.5-trt8.5-driver535.98-ubuntu18.04:4.0`

- 压缩镜像

`tar zcvf nvidia_cuda11.8-cudnn8.5-trt8.5-driver535.98-ubuntu18.04_4.0.tgz nvidia_cuda11.8-cudnn8.5-trt8.5-driver535.98-ubuntu18.04_4.0.tar`

## 加载

- 解压镜像

`tar zxvf nvidia_cuda11.8-cudnn8.5-trt8.5-driver535.98-ubuntu18.04_4.0.tgz`

- 从文件载入镜像

`docker load --input nvidia_cuda11.8-cudnn8.5-trt8.5-driver535.98-ubuntu18.04_4.0.tar`

## 启动
`docker run --restart=always --privileged=true --network=host -v /etc/localtime:/etc/localtime  -v /d1:/d1  -it --gpus all -e NVIDIA_DRIVER_CAPABILITIES=compute,utility,video -d nvidia_cuda11.8-cudnn8.5-trt8.5-driver535.98-ubuntu18.04:4.0  /bin/bash /start.sh`

> 备忘：已启动的docker，可通过` docker update --restart=always <id>` 更新重启参数 
