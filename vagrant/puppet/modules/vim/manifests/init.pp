class vim {
    package { "vim":
        ensure => installed,
    }

    file { "/home/vagrant/.vimrc":
        group  => "vagrant",
        mode   => "0644",
        owner  => "vagrant",
        source => "puppet:///modules/vim/vimrc",
    }

    vcsrepo { "/home/vagrant/.vim/bundle/Vundle.vim":
        ensure   => present,
        provider => git,
        source   => "git://github.com/gmarik/Vundle.vim.git",
        user     => "vagrant",
    }
}
