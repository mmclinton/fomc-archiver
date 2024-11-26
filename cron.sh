#!/usr/bin/env bash

USER=$(whoami)

log_file="/var/log/fomc.log"

if [ ! -w "$log_file" ]; then
    sudo touch "$log_file"
    sudo chmod 664 "$log_file"
    sudo chown "$USER" "$log_file"
fi

cron_job="0 5 * * * /usr/local/bin/fomc --update && echo \"\$(date): Job completed successfully\" >> /home/$USER/.config/fomc/fomc.log && tail -n 100 /home/$USER/.config/fomc/fomc.log > /home/$USER/.config/fomc/fomc.log.tmp && mv /home/$USER/.config/fomc/fomc.log.tmp /home/$USER/.config/fomc/fomc.log"

existing_cron=$(sudo crontab -u "$USER" -l 2>/dev/null | grep -F "$cron_job")

if [ -z "$existing_cron" ]; then
    (sudo crontab -u "$USER" -l 2>/dev/null; echo "$cron_job") | sudo crontab -u "$USER" -
    echo "A cron job was added to update the database every Monday at 5:00 AM for user $USER."
else
    echo "Cron job already exists. No changes made, and everything looks fine!"
fi
