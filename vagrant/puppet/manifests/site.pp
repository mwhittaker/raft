# create a new run stage to ensure certain modules are included first
stage { 'pre':
  before => Stage['main']
}

# add the baseconfig module to the new 'pre' run stage
class { 'baseconfig':
  stage => 'pre'
}

# set global exec path
Exec { path => [ "/bin", "/sbin/", "/usr/bin/", "/usr/sbin", "/usr/local/bin", "/usr/local/sbin" ] }

include baseconfig
include git
include gpp
include vim
include gvim
include tmux
include rust
include capnproto
include capnprotorust
