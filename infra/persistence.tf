resource "kubernetes_namespace" "persistence" {
    metadata {
        name = "persistence"
    }
}

module "mongo" {
    source = "./modules/base_service"
    service_name = "mongo"
    parent_namespace = "persistence"
    image_tag = "mongo:latest"
}
