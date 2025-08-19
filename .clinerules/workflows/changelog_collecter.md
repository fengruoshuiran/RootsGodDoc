````markdown
# 更新变更日志工作流程

```mermaid
graph TD
    A[开始] --> B[读取version.md获取版本号]
    B --> C[识别修改的文件]
    C --> D[读取文件内容]
    D --> E{已有变更日志?}
    E -->|是| F[记录已有变更]
    E -->|否| G[使用git diff获取变更]
    G --> H[AI总结变更内容]
    H --> I[写入文件CHANGELOG]
    I --> J[更新CHANGELOG.md]
    C --> K[验证SUMMARY.md]
    J --> L[完成]
    K --> L
````

## 详细分步指南

1. __获取当前版本__

   - 使用read_file工具读取.clinerules/version.md
   - 示例function call:
     ```xml
     <read_file>
     <path>.clinerules/version.md</path>
     </read_file>
     ```

2. __识别修改的文件__
   - 使用execute_command执行git add .确保所有文件被正确追踪
   - 使用execute_command执行git status
   - 示例function call:
     ```xml
     <execute_command>
     <command>git status --porcelain</command>
     <requires_approval>false</requires_approval>
     </execute_command>
     ```
   - 解析输出获取修改的文件路径

3. __读取文件内容__

   ```xml
   <read_file>
   <path>src/天赋/兔子脚.md</path>
   </read_file>
   ```

4. __检查已有变更日志__

   - 如果文件已有完整的变更日志，直接记录
   - 示例已有变更格式：
     ```javascript
     ## CHANGELOG

     - [1.1.7] 修改了触发条件
     - [1.1.6] 新增了特殊效果
     ```

5. __使用git diff获取变更(若无日志)__

   ```xml
   <execute_command>
   <command>git diff --unified=0 src/天赋/兔子脚.md</command>
   <requires_approval>false</requires_approval>
   </execute_command>
   ```

   - 解析diff输出获取具体变更

6. __AI总结变更内容__

   - 总结规则:

     - 识别新增/修改/删除的内容
     - 提取关键变更描述
     - 保持简洁一致的格式

   - 示例输出:
     ```javascript
     - 提高了Cost数值从1→2
     - 新增了特殊效果描述
     ```

7. __写入文件自身的变更__

   - 特别注意，版本日志规范是倒序的
   - 读取整个文件内容，添加变更后完整重写
   - 示例:
     ```xml
     <write_to_file>
     <path>src/天赋/兔子脚.md</path>
     <content>
     [原文件完整内容]

     ## CHANGELOG

     ### [1.1.7] - 2025-08-18

     - 提高了Cost数值从1→2
     - 新增了特殊效果描述
     ```
