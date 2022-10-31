toy lowcode engine

very early prototyping phase

the experiment with this project is to see whether the the following concept is technically realistic and/or useful

* define a set of abstract composable primitives including
    * data models / schemas
    * persistent storage
    * communication surfaces
    * logical effects
    * runtimes
    * infrastructure constructs
* such that
    * schemas, logic, and infrastructure are easily managed together
    * they can be composed together to create resuable units
    * they can be managed with high abstraction via some visual platform
    * source control is the source of truth
    * rather than trying to handle complex cases automatically, a streamlined escape hatch to just writing custom code is natural

the rationale is most problems are engineering time constrainted, but high-abstraction or no-code system that reduce engineering time
investment up front often limit long-term evolution
