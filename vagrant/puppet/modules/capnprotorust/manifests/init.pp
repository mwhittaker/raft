class capnprotorust {
    vcsrepo { "/home/vagrant/capnproto-rust":
        ensure   => present,
        user     => "vagrant",
        provider => git,
        source   => "git://github.com/dwrensha/capnproto-rust.git",
    }   
    
    exec { "make-capnprotorust":
        command => "make",
        user    => "vagrant",
        require => Exec["install-rust"],
        cwd     => "/home/vagrant/capnproto-rust",
    }

    Vcsrepo["/home/vagrant/capnproto-rust"] -> Exec["make-capnprotorust"]
}
