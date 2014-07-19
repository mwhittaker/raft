include git

class rust {
    # rust language
    exec { 'wget-rust': 
        command => "wget http://rust-lang.org/rustup.sh",
        cwd     => "/home/vagrant",
        creates => "/home/vagrant/rustup.sh",
    }
     
    exec { 'chmod-rust':
        command => "chmod 755 rustup.sh",
        cwd     => "/home/vagrant",
        require => Exec["wget-rust"],
    }
    
    exec { "install-rust":
        command  => "/home/vagrant/rustup.sh",
        cwd      => "/home/vagrant",
        timeout  => 3600,
        require  => Exec["chmod-rust"],
        creates  => "/usr/local/bin/rustc",
    }
      
    # raft
    vcsrepo { "/home/vagrant/raft":
        ensure   => present,
        user     => "vagrant",
        provider => git,
        source   => "git@github.com:mwhittaker/raft.git",
    }
}
