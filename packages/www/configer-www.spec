# Generated: Sun May 29 01:07:37 AM EDT 2022
Name    : configer-www
Version : 0.1.%{?build_number}%{!?build_number:x}
Release : 1%{dist}
Summary : Nginx module for ConfigerIO

Requires: nginx
BuildArch : x86_64
Packager  : PoiXson <support@poixson.com>
License   : GPLv3
URL       : https://configer.io

Prefix: %{_datadir}/configer
%define _rpmfilename  %%{NAME}-%%{VERSION}-%%{RELEASE}.%%{ARCH}.rpm

%description
Nginx module for ConfigerIO



### Install ###
%install
echo
echo "Install.."

# delete existing rpm's
%{__rm} -fv --preserve-root  "%{_rpmdir}/%{name}-"*.rpm

# create dirs
%{__install} -d -m 0755  \
	"%{buildroot}%{_bindir}/"                     \
	"%{buildroot}%{_sysconfdir}/systemd/system/"  \
	"%{buildroot}%{prefix}/"                      \
	"%{buildroot}%{prefix}/templates/www/"        \
		|| exit 1

# copy files
\pushd "%{_topdir}/../" >/dev/null || exit 1
	if [[ -d target/release/ ]]; then
		%{__install} -m 0755  "target/release/configer-www"  "%{buildroot}%{_bindir}/"  || exit 1
	else
		%{__install} -m 0755  "target/debug/configer-www"    "%{buildroot}%{_bindir}/"  || exit 1
	fi
	# templates
	%{__install} -m 0755  "templates/"*.tpl  "%{buildroot}%{prefix}/templates/www/"  || exit 1
	# systemd
	%{__install} -m 0755  "nginx.service"  "%{buildroot}%{_sysconfdir}/systemd/system/"  || exit 1
\popd >/dev/null



### Files ###
%files
%defattr(0400, configer, configer, 0500)
%attr(0500,root,root)  %{_bindir}/configer-www
%{_sysconfdir}/systemd/system/nginx.service
%{prefix}/templates/www/etc-nginx-nginx.conf.tpl
%{prefix}/templates/www/etc-nginx-conf.d-user.conf.tpl
