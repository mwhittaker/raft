class baseconfig {
    exec { "apt-get update":
        command => "apt-get update",
    }

    file { "/home/vagrant/.bashrc":
        group => "vagrant",
        mode  => "0644",
        owner => "vagrant",
        source => "puppet:///modules/baseconfig/bashrc",
    }
}
