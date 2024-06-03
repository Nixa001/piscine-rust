#!/bin/bash
git add .
echo "Your commit ..."
read commit

git commit -m "$commit"

# Pousser vers Gitea
git push 
# origin main
git branch -m master main  
# Pousser vers GitHub
git push github main
git branch -m master  