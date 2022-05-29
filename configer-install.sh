#!/usr/bin/bash
##==============================================================================
## Copyright (c) 2021-2022 PoiXson, Mattsoft
## <https://poixson.com> <https://mattsoft.net>
## Released under the GPL 3.0
##
## Description: ConfigerIO service management system
##
## Example:
## > wget -O configer-install.sh https://configer.io/install.sh
## > sh configer-install.sh
##
## This program is free software: you can redistribute it and/or modify
## it under the terms of the GNU General Public License as published by
## the Free Software Foundation, either version 3 of the License, or
## (at your option) any later version.
##
## This program is distributed in the hope that it will be useful,
## but WITHOUT ANY WARRANTY; without even the implied warranty of
## MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
## GNU General Public License for more details.
##
## You should have received a copy of the GNU General Public License
## along with this program.  If not, see <http://www.gnu.org/licenses/>.
##==============================================================================
# configer-install.sh
clear
echo



PXN_RELEASE_URL="https://yum.poixson.com/latest.rpm"
REMI_RELEASE_RHEL_URL="http://rpms.remirepo.net/enterprise/"
REMI_RELEASE_FEDORA_URL="http://rpms.remirepo.net/fedora/"
EPEL_RELEASE_9_URL="https://ftp-osl.osuosl.org/pub/fedora-epel/9/Everything/x86_64/Packages/e/epel-release-9-2.el9.noarch.rpm"
EPEL_RELEASE_8_URL="https://ftp-osl.osuosl.org/pub/fedora-epel/8/Everything/x86_64/Packages/e/epel-release-8-15.el8.noarch.rpm"
EPEL_RELEASE_7_URL="https://download-ib01.fedoraproject.org/pub/epel/7/x86_64/Packages/e/epel-release-7-14.noarch.rpm"



export YES=0
export NO=1

NUM_COLORS=$( tput colors 2>/dev/null )
if [[ $NUM_COLORS == ?(-)+([0-9]) ]] \
&& [[ $NUM_COLORS -ge 8 ]]; then
	COLOR_BLACK='\e[0;30m'
	COLOR_BLUE='\e[0;34m'
	COLOR_GREEN='\e[0;32m'
	COLOR_CYAN='\e[0;36m'
	COLOR_RED='\e[0;31m'
	COLOR_PURPLE='\e[0;35m'
	COLOR_BROWN='\e[0;33m'
	COLOR_LIGHTGRAY='\e[0;37m'
	COLOR_DARKGRAY='\e[1;30m'
	COLOR_LIGHTBLUE='\e[1;34m'
	COLOR_LIGHTGREEN='\e[1;32m'
	COLOR_LIGHTCYAN='\e[1;36m'
	COLOR_LIGHTRED='\e[1;31m'
	COLOR_LIGHTPURPLE='\e[1;35m'
	COLOR_YELLOW='\e[1;33m'
	COLOR_WHITE='\e[1;37m'
	COLOR_RESET='\e[0m'
else
	COLOR_BLACK=''
	COLOR_BLUE=''
	COLOR_GREEN=''
	COLOR_CYAN=''
	COLOR_RED=''
	COLOR_PURPLE=''
	COLOR_BROWN=''
	COLOR_LIGHTGRAY=''
	COLOR_DARKGRAY=''
	COLOR_LIGHTBLUE=''
	COLOR_LIGHTGREEN=''
	COLOR_LIGHTCYAN=''
	COLOR_LIGHTRED=''
	COLOR_LIGHTPURPLE=''
	COLOR_YELLOW=''
	COLOR_WHITE=''
	COLOR_RESET=''
fi



function error_msg() {
	echo "$*" >&2
}
function notice() {
	echo -e "${COLOR_LIGHTCYAN} [NOTICE] ${COLOR_RESET}$*" >&2
}
function warning() {
	echo -e "${COLOR_LIGHTRED} [WARNING] ${COLOR_RESET}$*" >&2
}
function failure() {
	echo -e "${COLOR_RED} [FAILURE] ${COLOR_RESET}$*" >&2
}

function title() {
	LONGEST_LEN=1
	for LINE in "$@"; do
		local LEN=${#LINE}
		if [ $LEN -gt $LONGEST_LEN ]; then
			LONGEST_LEN=$LEN
		fi
	done
	local _A=$(($LONGEST_LEN+8))
	local _B=$(($LONGEST_LEN+2))
	# format C
	for LINE in "${@}"; do
		local _S=$(($_B-${#LINE}))
		echo -ne "${COLOR_BROWN} [ ${LINE}"; eval "printf ' '%.0s {2..$_S}"; echo -e "]${COLOR_RESET}"
	done
}



OS_NAME=$(grep ^ID= "/etc/os-release")
OS_NAME=${OS_NAME#*=}
OS_NAME=${OS_NAME//\"}

OS_VERSION=$(grep ^VERSION_ID= "/etc/os-release")
OS_VERSION=${OS_VERSION#*=}
OS_VERSION=${OS_VERSION//\"}

if [[ -z $OS_NAME ]]; then
	failure "OS type not detected."
	failure ; exit 1
fi
if [[ -z $OS_VERSION ]]; then
	failure "OS version not detected."
	failure ; exit 1
fi



# CentOS / Rocky
if [[ "$OS_NAME" == "centos" ]] || [[ "$OS_NAME" == "rocky" ]]; then
	if   [[ "$OS_VERSION" == "9."* ]]; then
		OS_TITLE="CentOS/Rocky 9"
		EPEL_RELEASE_URL="$EPEL_RELEASE_9_URL"
		REMI_RELEASE_URL="${REMI_RELEASE_RHEL_URL}remi-release-9.rpm"
	elif [[ "$OS_VERSION" == "8."* ]]; then
		OS_TITLE="CentOS/Rocky 8"
		EPEL_RELEASE_URL="$EPEL_RELEASE_8_URL"
		REMI_RELEASE_URL="${REMI_RELEASE_RHEL_URL}remi-release-8.rpm"
	elif [[ "$OS_VERSION" == "7."* ]]; then
		OS_TITLE="CentOS/Rocky 7"
		EPEL_RELEASE_URL="$EPEL_RELEASE_7_URL"
		REMI_RELEASE_URL="${REMI_RELEASE_RHEL_URL}remi-release-7.rpm"
	else
		failure "Unsupported OS version: $OS_NAME $OS_VERSION"
		failure ; exit 1
	fi

# Fedora
elif [[ "$OS_NAME" == "fedora" ]]; then
	if   [[ "$OS_VERSION" == "36" ]]; then
		OS_TITLE="Fedora 36"
		REMI_RELEASE_URL="${REMI_RELEASE_FEDORA_URL}remi-release-36.rpm"
	elif [[ "$OS_VERSION" == "35" ]]; then
		OS_TITLE="Fedora 35"
		REMI_RELEASE_URL="${REMI_RELEASE_FEDORA_URL}remi-release-35.rpm"
	elif [[ "$OS_VERSION" == "34" ]]; then
		OS_TITLE="Fedora 34"
		REMI_RELEASE_URL="${REMI_RELEASE_FEDORA_URL}remi-release-34.rpm"
	else
		failure "Unsupported OS version: $OS_NAME $OS_VERSION"
		failure ; exit 1
	fi
fi

# unsupported os
if [[ -z $REMI_RELEASE_URL ]]; then
	failure "Unsupported OS: $OS_NAME $OS_VERSION"
	failure ; exit 1
fi



function DisplayHeader() {
	echo -e " ${COLOR_BLUE}╔═╗┌─┐┌┐┌┌─┐┬┌─┐┌─┐┬─┐ ╦╔═╗${COLOR_RESET} "
	echo -e " ${COLOR_BLUE}║  │ ││││├┤ ││ ┬├┤ ├┬┘ ║║ ║${COLOR_RESET} "
	echo -e " ${COLOR_BLUE}╚═╝└─┘┘└┘└  ┴└─┘└─┘┴└─*╩╚═╝${COLOR_RESET} "
	echo
}

function DisplayHelp() {
	echo -e "${COLOR_BROWN}Usage:${COLOR_RESET}"
	echo    "  install.sh [options]"
	echo
	echo -e "${COLOR_BROWN}Options:${COLOR_RESET}"
	echo -e "  ${COLOR_GREEN}-y, --yes${COLOR_RESET}                 Automatically answer yes for all questions"
	echo -e "  ${COLOR_GREEN}-D, --dry${COLOR_RESET}                 Dry-run, no changes will be performed by actions"
	echo
	echo -e "  ${COLOR_GREEN}-v, --verbose${COLOR_RESET}             Enable debug logs"
	echo -e "  ${COLOR_GREEN}-h, --help${COLOR_RESET}                Display this help message and exit"
	echo
	exit 1
}



IS_AUTO=$NO
IS_DRY=$NO
VERBOSE=$NO
while [ $# -gt 0 ]; do
	case "$1" in
	# auto install
	-y|--yes|--assumeyes|--auto)
		IS_AUTO=$YES
	;;
	# dry mode
	-D|--dry)
		IS_DRY=$YES
	;;
	# verbose logging
	-v|--verbose)
		VERBOSE=$YES
	;;
	-h|--help)
		DisplayHelp
		exit 1
	;;
	*)
		failure "Unknown argument: $1"
		failure
		DisplayHelp
		exit 1
	;;
	esac
	\shift
done



# run only as root
if [[ $EUID -ne 0 ]]; then
	failure "Please run this script as root"
	failure ; exit 1
fi



notice "Detected: $OS_TITLE"
if [[ $IS_DRY -eq $YES ]]; then
	notice "Dry-run"
fi
if [[ $IS_AUTO -eq $YES ]]; then
	notice "Auto-yes"
fi
echo



function doInstall() {
	if [[ -z $1 ]]; then
		failure "No package argument provided to doInstall()"
		failure ; exit 1
	fi
	if [[ -z $2 ]]; then
		local TITLE="${1/ /, }"
	else
		local TITLE="$2"
	fi
	title "Installing $TITLE.."
	if [[ $IS_AUTO -eq $YES ]]; then
		local Y_FLAG=" -y"
	else
		local Y_FLAG=""
	fi
	if [[ $IS_DRY -eq $YES ]]; then
		echo "Skipping.."
	fi
	echo " > dnf$Y_FLAG install  $1"
	if [[ $IS_DRY -ne $YES ]]; then
		\dnf install $Y_FLAG  $1  || exit 1
	fi
	echo
}
function doUpdate() {
	title "Updating.."
	if [[ $IS_AUTO -eq $YES ]]; then
		local Y_FLAG=" -y"
	else
		local Y_FLAG=""
	fi
	if [[ $IS_DRY -eq $YES ]]; then
		echo "Skipping.."
	fi
	echo " > dnf$Y_FLAG update"
	if [[ $IS_DRY -ne $YES ]]; then
		\dnf update $Y_FLAG  || exit 1
	fi
	echo
}



# pxn.repo
if [[ ! -f /etc/yum.repos.d/pxn.repo ]]; then
	doInstall  "$PXN_RELEASE_URL"  "pxn repo"
fi

# epel repo
if [[ ! -z $EPEL_RELEASE_URL ]]; then
	if [[ ! -f /etc/yum.repos.d/epel.repo ]]; then
		doInstall  "$EPEL_RELEASE_URL"  "epel repo"
	fi
fi

# remi repo
if [[ ! -f /etc/yum.repos.d/remi.repo ]]; then
	doInstall  "$REMI_RELEASE_URL"  "remi repo"
fi



# update system
doUpdate


# enable remi repo


doInstall  "pxnscripts"  "pxnScripts"


echo "UNFINISHED"
exit 1
