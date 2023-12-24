@echo off
::切换到目录
cd /d D:\code\master-rust
git pull
git add -A .
git add *.md
git add *.py
git commit -m "update"
git push