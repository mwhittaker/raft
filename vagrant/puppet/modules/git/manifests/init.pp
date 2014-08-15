class git {
    package { "git":
        ensure => installed,
    }

    file { "/home/vagrant/.gitconfig":
        group => "vagrant",
        mode  => "0644",
        owner => "vagrant",
        source => "puppet:///modules/git/gitconfig",
    }
}
