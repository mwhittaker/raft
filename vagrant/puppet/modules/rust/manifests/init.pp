include git

class rust {
    exec { "wget-rust": 
        command => "wget http://rust-lang.org/rustup.sh",
        creates => "/home/vagrant/rustup.sh",
        cwd     => "/home/vagrant",
    }
     
    exec { "chmod-rust":
        command => "chmod 755 rustup.sh",
        cwd     => "/home/vagrant",
        require => Exec["wget-rust"],
    }
    
    exec { "install-rust":
        command  => "/home/vagrant/rustup.sh",
        creates  => "/usr/local/bin/rustc",
        cwd      => "/home/vagrant",
        require  => Exec["chmod-rust"],
        timeout  => 0,
    }
}
