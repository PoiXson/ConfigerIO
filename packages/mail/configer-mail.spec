# Generated: Sat May  7 11:04:55 PM EDT 2022
Name    : configer-mail
Version : 0.1
Release : 1
Summary : Postfix module for ConfigerIO

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
	"%{buildroot}%{prefix}/"                      \
	"%{buildroot}%{prefix}/templates/mail/"       \
		|| exit 1

# copy files
\pushd "%{_topdir}/../" >/dev/null || exit 1
	%{__install} -m 0755  \
		"target/debug/configer-mail"  \
		"%{buildroot}%{_bindir}/"     \
			|| exit 1
	# templates
	%{__install} -m 0755  \
		"templates/"*.tpl                        \
		"%{buildroot}%{prefix}/templates/mail/"  \
			|| exit 1
	# systemd
	%{__install} -m 0755  \
		"postfix.service"                             \
		"%{buildroot}%{_sysconfdir}/systemd/system/"  \
			|| exit 1
\popd >/dev/null



### Files ###
%files
%defattr(0400, configer, configer, 0500)
%attr(0500,root,root)  %{_bindir}/configer-mail
%{_sysconfdir}/systemd/system/postfix.service
%{prefix}/templates/mail/etc-postfix-main.cf.tpl
