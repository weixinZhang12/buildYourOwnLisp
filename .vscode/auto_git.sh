#!/bin/bash
# 添加所有更改到暂存区
git add .
# 自动提交（自定义提交信息）
git commit -m "Auto-sync: $(date +'%Y-%m-%d %H:%M:%S')"
# 拉取远程更新并自动rebase（减少合并提交）
git pull --rebase origin $(git branch --show-current)
# 推送更改
git push origin $(git branch --show-current)
echo "✅ 同步完成！"