# Generated: Thu Jun  2 07:14:03 AM EDT 2022
Name    : configer-fw
Version : 0.1.%{?build_number}%{!?build_number:x}
Release : 1%{dist}
Summary : Firewall module for ConfigerIO

Requires: iptables
BuildRequires: systemd-rpm-macros

BuildArch : x86_64
Packager  : PoiXson <support@poixson.com>
License   : GPLv3
URL       : https://configer.io

Prefix: %{_datadir}/configer
%define _rpmfilename  %%{NAME}-%%{VERSION}-%%{RELEASE}.%%{ARCH}.rpm

%description
Firewall module for ConfigerIO



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
	"%{buildroot}%{_presetdir}/"                  \
	"%{buildroot}%{prefix}/"                      \
	"%{buildroot}%{prefix}/templates/fw/"         \
		|| exit 1

# copy files
\pushd "%{_topdir}/../" >/dev/null || exit 1
	if [[ -d target/release/ ]]; then
		%{__install} -m 0755  "target/release/configer-fw"  "%{buildroot}%{_bindir}/"  || exit 1
	else
		%{__install} -m 0755  "target/debug/configer-fw"    "%{buildroot}%{_bindir}/"  || exit 1
	fi
	# templates
	%{__install} -m 0755  "templates/"*.tpl  "%{buildroot}%{prefix}/templates/fw/"  || exit 1
	# systemd
	%{__install} -m 0755  "fw.service"  "%{buildroot}%{_sysconfdir}/systemd/system/"  || exit 1
	%{__install} -m 0755  "5-fw.preset"  "%{buildroot}%{_presetdir}/"                 || exit 1
\popd >/dev/null



%post
%systemd_post  fw.service
if [[ "$1" -eq 1 ]]; then
	/usr/bin/systemctl stop     firewalld.service  || :
	/usr/bin/systemctl disable  firewalld.service  || :
	/usr/bin/systemctl stop     iptables.service   || :
	/usr/bin/systemctl disable  iptables.service   || :
	/usr/bin/systemctl start  fw.service  || :
fi

%preun
%systemd_preun  fw.service

%postun
%systemd_postun_with_restart  fw.service



### Files ###
%files
%defattr(0400, configer, configer, 0500)
%attr(0500,root,root)  %{_bindir}/configer-fw
%{_sysconfdir}/systemd/system/fw.service
%{_presetdir}/5-fw.preset
%{prefix}/templates/fw/etc-sysconfig-iptables.tpl
