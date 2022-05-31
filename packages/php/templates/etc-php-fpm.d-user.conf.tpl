; *** Generated by ConfigerIO ***
; {{{timestamp}}}
; {{{hostname}}}

[{{{user}}}]

chdir = /home/{{{user}}}/www

listen.mode = 0666
listen = /run/php-{{{user}}}.sock
listen.owner = {{{user}}}
listen.group = {{{user}}}

pm = ondemand
pm.max_children = 10
pm.start_servers = 1
pm.min_spare_servers = 1
pm.max_spare_servers = 5
pm.process_idle_timeout = 60s
pm.max_requests = 20
