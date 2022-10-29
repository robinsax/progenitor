variable "service_versions" {
    description = "Service versions"
    type = map(string)
}

resource "kubernetes_namespace" "surface" {
    metadata {
        name = "surface"
    }
}

module "frontend" {
    source = "./modules/prg_service"
    service_name = "frontend"
    service_version = var.service_versions.frontend
    parent_namespace = "surface"
    source_dir = "./frontend"
    ingress_route = "/*"
}

module "api" {
    source = "./modules/prg_service"
    service_name = "api"
    service_version = var.service_versions.api
    parent_namespace = "surface"
    source_dir = "./api"
    ingress_route = "/api/*"
    internal_port = 8000
}
