variable "service_name" {
    description = "Service name"
    type = string
}

variable "parent_namespace" {
    description = "Parent namespace"
    type = string
}

variable "image_tag" {
    description = "Container tag"
    type = string
}

variable "internal_port" {
    description = "Internal port"
    default = 8080
    type = number
}

variable "ingress_route" {
    description = "Ingress route, none means no ingress rule"
    default = ""
    type = string
}
