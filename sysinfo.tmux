#!/usr/bin/env bash

set -x

CURRENT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"


get_tmux_option() {
	local option="$1"
	local default_value="$2"
	local option_value=$(tmux show-option -gqv "$option")
	if [ -z "$option_value" ]; then
		echo "$default_value"
	else
		echo "$option_value"
	fi
}

set_tmux_option() {
	local option="$1"
	local value="$2"
	tmux set-option -gq "$option" "$value"
}

ping_host=$(get_tmux_option '@tmux_ping_host' '192.168.8.1')
check_url=$(get_tmux_option '@tmux_check_url' 'https://www.google.com/')

status_script="#($CURRENT_DIR/bin/tmux-curl $check_url) #($CURRENT_DIR/bin/tmux-ping $ping_host) #($CURRENT_DIR/bin/tmux-sysinfo)"

update_tmux_option() {
	local option="$1"
	local option_value="$(get_tmux_option "$option")"
	# replace interpolation string with a script to execute
	# local new_option_value="${option_value/$status_interpolation_string/$status_script}"
	local new_option_value="$status_script ${option_value}"
	set_tmux_option "$option" "$new_option_value"
}

main() {
    update_tmux_option "status-right"
}
main
