resource "null_resource" "image_load" {
    triggers = {
        tag = var.image_tag
    }
    provisioner "local-exec" {
        interpreter=["/bin/bash", "-c"]
        command = "minikube image load ${var.image_tag}"
    }
}

resource "kubernetes_service" "service" {
    metadata {
        name = var.service_name
        namespace = var.parent_namespace
    }
    spec {
        selector = {
            parent = var.service_name
        }
        port {
            port = var.internal_port
            target_port = var.internal_port
            protocol = "TCP"
        }
        type = "ClusterIP"
    }
}

resource "kubernetes_ingress_v1" "ingress" {
    count = var.ingress_route == "" ? 0 : 1
    metadata {
        name = "${var.service_name}-ingress"
        namespace = var.parent_namespace
        annotations = {
            "nginx.ingress.kubernetes.io/rewrite-target" = "/$1"
        }
    }
    spec {
        rule {
            http {
                path {
                    path = var.ingress_route
                    backend {
                        service {
                            name = var.service_name
                            port {
                                number = var.internal_port
                            }
                        }
                    }
                }
            }
        }
    }
    depends_on = [
        kubernetes_service.service
    ]
}

# TODO: replication controller etc, obviously this is stupid currently.
resource "kubernetes_pod" "runtime" {
    metadata {
        name = "${var.service_name}-singleton"
        namespace = var.parent_namespace
        labels = {
            parent = var.service_name
        }
    }
    spec {
        container {
            image = var.image_tag
            name  = "${var.service_name}-container"
            port {
                container_port = var.internal_port
            }
        }
    }
    depends_on = [
        null_resource.image_load,
        kubernetes_service.service
    ]
}
