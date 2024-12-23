#!/bin/bash
git lfs install;
git config lfs.allowincompletepush true;
git config --global core.autocrlf input;
git add .;
echo -n "Please enter the update message and press Enter:"
read update_message
if [ -z "$update_message" ]; then
    echo "The update message cannot be empty. Please rerun the script and provide a valid update message."
    echo "Press Enter to exit...";
    read -n 1;
    exit 1
fi
git commit -m "feat:$update_message";
git push ltpp master;
echo -e "\e[32mltpp push finish\e[0m";
git push origin master;
echo -e "\e[32morigin push finish\e[0m";
echo "Press Enter to continue...";
read -n 1;