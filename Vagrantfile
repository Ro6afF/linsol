# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "envimation/ubuntu-xenial"

  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install rustc -y
  SHELL
end
