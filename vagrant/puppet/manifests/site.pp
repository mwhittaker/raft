# create a new run stage to ensure certain modules are included first
stage { 'pre':
  before => Stage['main']
}

# add the baseconfig module to the new 'pre' run stage
class { 'baseconfig':
  stage => 'pre'
}

# set global exec path
Exec { path => [ "/bin", "/sbin/", "/usr/bin/", "/usr/sbin" ] }

include baseconfig
include git
include vim
include gvim
include tmux
include rust
