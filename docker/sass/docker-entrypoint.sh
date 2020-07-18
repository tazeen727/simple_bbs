#!/bin/sh
sass -s compressed --watch stylesheets:static/css &

tail -f /dev/null
