terraform {
    required_providers {
        kubernetes = {
            source = "hashicorp/kubernetes"
            version = "2.14.0"
        }
    }
}

provider "kubernetes" {
    config_path    = "~/.kube/config"
    config_context = "minikube"
}

resource "null_resource" "tunnel" {
    triggers = {
        retrigger = timestamp()
    }
    provisioner "local-exec" {
        command = "gnome-terminal -- /bin/bash -c 'minikube tunnel --cleanup=true'"
    }
}
