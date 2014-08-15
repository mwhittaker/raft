class tmux {
    package { "tmux":
        ensure => installed,
    }
    
    file { "/home/vagrant/.tmux.conf":
        group => "vagrant",
        mode  => "0644",
        owner => "vagrant",
        source => "puppet:///modules/tmux/tmux.conf",
    }
}
