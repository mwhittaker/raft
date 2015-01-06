# Vagrant #
This directory contains vagrant files and puppet scripts to boot and configure
a virtual machine that comes with `rustc`, `cargo`, and a few other goodies.

## Getting Started ##
First, make sure you've initialized this repository's submodules. If you
haven't yet, you can run the following two commands.

    $ git submodule init
    $ git submodule update

Next, head over to [`puppet/modules/git/files`](puppet/modules/git/files) and
add your favorite `.gitconfig` file, but name it `gitconfig` without the
leading period. Now, we're ready to boot the machine with `vagrant up`.

    $ vagrant up

After the machine boots, you can ssh into it by using `vagrant ssh`.

    $ vagrant ssh

You can suspend, halt, and destroy the machine with `vagrant susped`, `vagrant
halt`, and `vagrant destroy`.

## More Configuration ##
After you've booted, provisioned, and ssh'ed into the machine, there are a few
things you may want to do.

- Clone the raft repository.

        $ git clone https://github.com/mwhittaker/raft

- Improve vim.
       
        $ vim +PluginInstall +qall

- Configure vim, tmux, bashrc, etc.

        $ git clone https://github.com/mwhittaker/dotfiles
        $ cd dotfiles
        $ ./install.sh
