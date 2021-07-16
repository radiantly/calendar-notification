#!/bin/bash

rm {/tmp,$XDG_RUNTIME_DIR}/calendar_notification_month
hyperfine --warmup 3 './i3blocks/actions/date.sh next' 'calendar-notification next'
