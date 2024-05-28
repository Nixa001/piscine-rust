#!/bin/bash

# Pousser vers Gitea
git push 
# origin main

git branch -m master main  
# Pousser vers GitHub
git push github main

git branch -m master  