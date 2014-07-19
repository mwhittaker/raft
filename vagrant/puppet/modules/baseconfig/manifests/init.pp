class baseconfig {
    exec { 'apt-get update':
        command => 'apt-get update';
    }

    file { '/home/vagrant/.bashrc':
        owner => 'vagrant',
        group => 'vagrant',
        mode  => '0644',
        source => 'puppet:///modules/baseconfig/bashrc';
    }

    file { '/home/vagrant/.tmux.conf':
        owner => 'vagrant',
        group => 'vagrant',
        mode  => '0644',
        source => 'puppet:///modules/baseconfig/tmux.conf';
    }

    file { '/home/vagrant/.ssh/known_hosts':
        owner => 'vagrant',
        group => 'vagrant',
        mode  => '0644',
        source => 'puppet:///modules/baseconfig/known_hosts';
    }
}
