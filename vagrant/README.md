# Vagrant #
This directory contains vagrant files and puppet scripts to boot and configure
a virtual machine that comes with `rustc`, `cargo`, and a few other goodies.

## Getting Started ##
First, head over to [`puppet/modules/git/files`](puppet/modules/git/files) and
add your favorite `.gitconfig` file, but name it `gitconfig` without the
leading period. Now, we're ready to boot the machine with `vagrant up`.

    $ vagrant up

After the machine boots, you can ssh into it by using `vagrant ssh`.

    $ vagrant ssh

You can suspend, halt, and destroy the machine with `vagrant susped`, `vagrant
halt`, and `vagrant destroy`.
