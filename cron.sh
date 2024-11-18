#!/usr/bin/env bash

USER=$(whoami)

cron_job="0 5 * * * /usr/local/bin/fomc --update"

existing_cron=$(sudo crontab -u "$USER" -l 2>/dev/null | grep -F "$cron_job")

if [ -z "$existing_cron" ]; then
    (sudo crontab -u "$USER" -l 2>/dev/null; echo "$cron_job") | sudo crontab -u "$USER" -
    echo "A cron job was added to update the database every Monday at 5:00 AM for user $USER."
else
    echo "Cron job already exists. No changes made, and everything looks fine!"
fi
