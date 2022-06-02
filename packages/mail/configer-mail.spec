# Generated: Thu Jun  2 07:15:08 AM EDT 2022
Name    : configer-mail
Version : 0.1.%{?build_number}%{!?build_number:x}
Release : 1%{dist}
Summary : Postfix module for ConfigerIO

Requires: postfix
BuildRequires: systemd-rpm-macros

BuildArch : x86_64
Packager  : PoiXson <support@poixson.com>
License   : GPLv3
URL       : https://configer.io

Prefix: %{_datadir}/configer
%define _rpmfilename  %%{NAME}-%%{VERSION}-%%{RELEASE}.%%{ARCH}.rpm

%description
Postfix module for ConfigerIO



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
	"%{buildroot}%{prefix}/templates/mail/"       \
		|| exit 1

# copy files
\pushd "%{_topdir}/../" >/dev/null || exit 1
	if [[ -d target/release/ ]]; then
		%{__install} -m 0755  "target/release/configer-mail"  "%{buildroot}%{_bindir}/"  || exit 1
	else
		%{__install} -m 0755  "target/debug/configer-mail"    "%{buildroot}%{_bindir}/"  || exit 1
	fi
	# templates
	%{__install} -m 0755  "templates/"*.tpl  "%{buildroot}%{prefix}/templates/mail/"  || exit 1
	# systemd
	%{__install} -m 0755  "postfix.service"  "%{buildroot}%{_sysconfdir}/systemd/system/"  || exit 1
	%{__install} -m 0755  "10-postfix.preset"  "%{buildroot}%{_presetdir}/"                || exit 1
\popd >/dev/null



%post
%systemd_post  postfix.service
if [[ "$1" -eq 1 ]]; then
	/usr/bin/systemctl start  postfix.service  || :
fi

%preun
%systemd_preun  postfix.service

%postun
%systemd_postun_with_restart  postfix.service



### Files ###
%files
%defattr(0400, configer, configer, 0500)
%attr(0500,root,root)  %{_bindir}/configer-mail
%{_sysconfdir}/systemd/system/postfix.service
%{_presetdir}/10-postfix.preset
%{prefix}/templates/mail/etc-postfix-main.cf.tpl
