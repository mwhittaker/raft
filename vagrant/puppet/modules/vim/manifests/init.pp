class vim {
    package { "vim":
        ensure => installed; 
    }

    file { '/home/vagrant/.vimrc':
        owner  => 'vagrant',
        group  => 'vagrant',
        mode   => '0644',
        source => 'puppet:///modules/vim/vimrc';
    }

    vcsrepo { "/home/vagrant/.vim/bundle/Vundle.vim":
        ensure   => present,
        user     => "vagrant",
        provider => git,
        source   => "git://github.com/gmarik/Vundle.vim.git",
    }
   
    exec { "PluginInstall":
        command     => "/usr/bin/vim +PluginInstall +qall",
        environment => ["HOME=/home/vagrant"],
        require     => Vcsrepo["/home/vagrant/.vim/bundle/Vundle.vim"],
        user    => "vagrant",
    }
}
