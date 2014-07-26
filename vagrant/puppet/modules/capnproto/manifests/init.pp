class capnproto {
    exec { "curl-capnp":
        command => "curl -O https://capnproto.org/capnproto-c++-0.4.1.tar.gz",
        user    => "vagrant",
        cwd     => "/home/vagrant",
        creates => "/home/vagrant/capnproto-c++-0.4.1.tar.gz",
    }

    exec { "tar-capnp":
        command => "tar zxf capnproto-c++-0.4.1.tar.gz",
        user    => "vagrant",
        cwd     => "/home/vagrant",
        creates => "/home/vagrant/capnproto-c++-0.4.1",
    }

    exec { "configure-capnp":
        command => "/home/vagrant/capnproto-c++-0.4.1/configure",
        user    => "vagrant",
        cwd     => "/home/vagrant/capnproto-c++-0.4.1",
    }

    exec { "make-check-capnp":
        command => "make check",
        user    => "vagrant",
        cwd     => "/home/vagrant/capnproto-c++-0.4.1",
    }

    exec { "make-install-capnp":
        command => "make install",
        cwd     => "/home/vagrant/capnproto-c++-0.4.1",
    }
    
    Class["gpp"]             ->
    Exec["curl-capnp"]       ->
    Exec["tar-capnp"]        ->
    Exec["configure-capnp"]  ->
    Exec["make-check-capnp"] ->
    Exec["make-install-capnp"] 
}
