# CHANGELOG收集者

这是一项流程相当明确的工作，除非出现了意料之外的情况，否则你必须严格按照流程进行

1. 读取 `.clinerules/version.md` 获取当前版本号
2. 使用 `git add . && git --no-pager diff --staged` 获取所有修改，整理其中设计文档在当前版本发生的改动
3. 读取 `src/CHANGELOG.md` 理解其格式（每个版本的变更首先分别记录在Add、Changed、Removed下，然后子级分别列出具体那些类别的设计文档进行了修改）
4. 修改 `src/CHANGELOG.md`，新增、修改、删除必须被正确分类
5. 读取 `src/SUMMARY.md` 理解其格式，这个是mdbook自动生成网页的关键部分，其中任何空白都是类似python语言般的拥有格式化语义的
6. 修改 `src/SUMMARY.md`，如果变更的设计文档在目录中不存在请添加到正确的位置，如果位置需要修改请移动到正确的位置
