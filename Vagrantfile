# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "envimation/ubuntu-xenial"
  config.vm.synced_folder ".", "/home/vagrant/linsol"
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install rustc -y
    apt-get install cargo -y
  SHELL
end
