locals {
    image_tag = "${var.service_name}:${var.service_version}"
}

resource "null_resource" "build" {
    triggers = {
        version = var.service_version
    }
    provisioner "local-exec" {
        interpreter=["/bin/bash", "-c"]
        command = "cd ../${var.source_dir} && ./bake.sh ${local.image_tag}"
    }
}

module "base" {
    source = "../base_service"
    service_name = var.service_name
    parent_namespace = var.parent_namespace
    image_tag = local.image_tag
    internal_port = var.internal_port
    ingress_route = var.ingress_route
    depends_on = [
        null_resource.build
    ]
}
