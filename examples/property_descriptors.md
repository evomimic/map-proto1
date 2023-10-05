## PropertyDescriptor for a String Property
```
PropertyDescriptor {
    header: TypeHeader {
        type_name: "simple_Boolean_Type_example",
        base_type: Boolean,
        description: "Simple Example Boolean Property Type description",
        version: SemanticVersion {
            major: 0,
            minor: 0,
            patch: 1,
        },
        is_dependent: true,
    },
    details: Boolean(
        BooleanDescriptor {
            is_fuzzy: false,
        },
    ),
}
```

## PropertyDescriptor for a Composite Property  
```
PropertyDescriptor {
    header: TypeHeader {
        type_name: "Simple__Composite_Type__with_scalar_properties",
        base_type: Composite,
        description: "Simple Composite Property Type description",
        version: SemanticVersion {
            major: 0,
            minor: 0,
            patch: 1,
        },
        is_dependent: true,
    },
    details: Composite(
        CompositeDescriptor {
            properties: PropertyDescriptorMap {
                properties: {
                    "a_boolean_property": PropertyDescriptorUsage {
                        description: "example boolean usage description",
                        descriptor: PropertyDescriptor {
                            header: TypeHeader {
                                type_name: "simple_Boolean_Type",
                                base_type: Boolean,
                                description: "Simple Boolean Property Type description",
                                version: SemanticVersion {
                                    major: 0,
                                    minor: 0,
                                    patch: 1,
                                },
                                is_dependent: true,
                            },
                            details: Boolean(
                                BooleanDescriptor {
                                    is_fuzzy: false,
                                },
                            ),
                        },
                        sharing: Dedicated,
                    },
                    "a_string_property": PropertyDescriptorUsage {
                        description: "example string usage description",
                        descriptor: PropertyDescriptor {
                            header: TypeHeader {
                                type_name: "simple__String_Type",
                                base_type: String,
                                description: "Simple String Property Type description",
                                version: SemanticVersion {
                                    major: 0,
                                    minor: 0,
                                    patch: 1,
                                },
                                is_dependent: true,
                            },
                            details: String(
                                StringDescriptor {
                                    min_length: 0,
                                    max_length: 2048,
                                },
                            ),
                        },
                        sharing: Dedicated,
                    },
                    "an_I64_property": PropertyDescriptorUsage {
                        description: "example string usage description",
                        descriptor: PropertyDescriptor {
                            header: TypeHeader {
                                type_name: "simple_I64_Integer_Type",
                                base_type: Integer,
                                description: "Simple Integer (I64) Property Type description",
                                version: SemanticVersion {
                                    major: 0,
                                    minor: 0,
                                    patch: 1,
                                },
                                is_dependent: true,
                            },
                            details: Integer(
                                IntegerDescriptor {
                                    format: I64,
                                    min_value: -9223372036854775808,
                                    max_value: 9223372036854775807,
                                },
                            ),
                        },
                        sharing: Dedicated,
                    },
                    
                },
            },
        },
    ),
}
```

