#!/bin/bash

rm /tmp/calendar_notification_month
hyperfine --warmup 3 './i3blocks/actions/date.sh next' 'calendar-notification prev'
