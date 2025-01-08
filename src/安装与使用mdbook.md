# 安装与使用mdbook

如果不仅是查看者而希望编辑设计文档，则需要遵循以下步骤来获取必要工具

参考[mdbook文档](https://rust-lang.github.io/mdBook/guide/installation.html)

## 安装rust

下载安装[rust](https://rust-lang.github.io/mdBook/index.html)

## 安装mdbook

打开cmd，执行
```
cargo install mdbook
```
等待安装完毕即可

本地的修改可以运行local_test.bat来查看

### markdown简介

mdbook使使用拓展的markdown规则编写的，对于markdown的基础语法，请[参阅](https://markdown.com.cn/basic-syntax/)

推荐使用vscode编辑，按需安装拓展

## 获取github权限

确定拥有[github](https://github.com/)账号，联系管理人员获取文档仓库的编辑权限

## 安装github工具

安装[github desktop](https://desktop.github.com/download/)，一个方便易用的官方桌面工具，用于在本地同步与编辑文档原始markdown文件

拉取项目，新建自己的分支进行编辑与提交。不要在主分支上直接进行编辑，因为这会频繁触发部署

> 作为替代的，也可以安装[git](https://git-scm.com/downloads/win)，安装vscode插件git graph进行交互

在编辑完成后，进行pull request，通过后提交将呈现在[文档网页](https://fengruoshuiran.github.io/RootsGodDoc/)中

Seija已安装！！！！
