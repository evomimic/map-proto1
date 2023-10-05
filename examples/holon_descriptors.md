## HolonDescriptor with No Properties  
```
HolonDescriptor {
    header: TypeHeader {
        type_name: "Holon_Type__with_no_properties",
        base_type: Holon,
        description: "A simple holon type that has no properties.",
        version: SemanticVersion {
            major: 0,
            minor: 0,
            patch: 1,
        },
        is_dependent: false,
    },
    properties: PropertyDescriptorMap {
        properties: {},
    },
}  
```

## HolonDescriptor with a Single Boolean Property  
```
HolonDescriptor {
    header: TypeHeader {
        type_name: "Holon_Type__with_single_boolean_property",
        base_type: Holon,
        description: "A simple holon type that has a single boolean property",
        version: SemanticVersion {
            major: 0,
            minor: 0,
            patch: 1,
        },
        is_dependent: false,
    },
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
        },
    },
}
```

## HolonDescriptor with 3 Scalar Properties  
```
HolonDescriptor {
    header: TypeHeader {
        type_name: "Holon_Type__with_scalar_properties",
        base_type: Holon,
        description: "A holon type that has a single property of each scalar property type.",
        version: SemanticVersion {
            major: 0,
            minor: 0,
            patch: 1,
        },
        is_dependent: false,
    },
    properties: PropertyDescriptorMap {
        properties: {
            "a_U8_property": PropertyDescriptorUsage {
                description: "example string usage description",
                descriptor: PropertyDescriptor {
                    header: TypeHeader {
                        type_name: "simple_U8_Integer_Type",
                        base_type: Integer,
                        description: "Simple Integer (U8) Property Type description",
                        version: SemanticVersion {
                            major: 0,
                            minor: 0,
                            patch: 1,
                        },
                        is_dependent: true,
                    },
                    details: Integer(
                        IntegerDescriptor {
                            format: U8,
                            min_value: 0,
                            max_value: 127,
                        },
                    ),
                },
                sharing: Dedicated,
            },
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
            
        },
    },
}

```

## HolonDescriptor with 1 Composite Property  
```

HolonDescriptor {
    header: TypeHeader {
        type_name: "Holon_Type__with_composite_properties",
        base_type: Holon,
        description: "A holon type that has a single property of a composite property type.",
        version: SemanticVersion {
            major: 0,
            minor: 0,
            patch: 1,
        },
        is_dependent: false,
    },
    properties: PropertyDescriptorMap {
        properties: {
            "a_composite_property": PropertyDescriptorUsage {
                description: "example composite usage description",
                descriptor: PropertyDescriptor {
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
                                    "a_U16_property": PropertyDescriptorUsage {
                                        description: "example string usage description",
                                        descriptor: PropertyDescriptor {
                                            header: TypeHeader {
                                                type_name: "simple_U16_Integer_Type",
                                                base_type: Integer,
                                                description: "Simple Integer (U16) Property Type description",
                                                version: SemanticVersion {
                                                    major: 0,
                                                    minor: 0,
                                                    patch: 1,
                                                },
                                                is_dependent: true,
                                            },
                                            details: Integer(
                                                IntegerDescriptor {
                                                    format: U16,
                                                    min_value: 0,
                                                    max_value: 32767,
                                                },
                                            ),
                                        },
                                        sharing: Dedicated,
                                    },
                                    "a_U32_property": PropertyDescriptorUsage {
                                        description: "example string usage description",
                                        descriptor: PropertyDescriptor {
                                            header: TypeHeader {
                                                type_name: "simple_U32_Integer_Type",
                                                base_type: Integer,
                                                description: "Simple Integer (U32) Property Type description",
                                                version: SemanticVersion {
                                                    major: 0,
                                                    minor: 0,
                                                    patch: 1,
                                                },
                                                is_dependent: true,
                                            },
                                            details: Integer(
                                                IntegerDescriptor {
                                                    format: U32,
                                                    min_value: 0,
                                                    max_value: 2147483648,
                                                },
                                            ),
                                        },
                                        sharing: Dedicated,
                                    },
                                    "a_U64_property": PropertyDescriptorUsage {
                                        description: "example string usage description",
                                        descriptor: PropertyDescriptor {
                                            header: TypeHeader {
                                                type_name: "simple_U64_Integer_Type",
                                                base_type: Integer,
                                                description: "Simple Integer (U64) Property Type description",
                                                version: SemanticVersion {
                                                    major: 0,
                                                    minor: 0,
                                                    patch: 1,
                                                },
                                                is_dependent: true,
                                            },
                                            details: Integer(
                                                IntegerDescriptor {
                                                    format: U64,
                                                    min_value: 0,
                                                    max_value: 9223372036854775807,
                                                },
                                            ),
                                        },
                                        sharing: Dedicated,
                                    },
                                    "a_U8_property": PropertyDescriptorUsage {
                                        description: "example string usage description",
                                        descriptor: PropertyDescriptor {
                                            header: TypeHeader {
                                                type_name: "simple_U8_Integer_Type",
                                                base_type: Integer,
                                                description: "Simple Integer (U8) Property Type description",
                                                version: SemanticVersion {
                                                    major: 0,
                                                    minor: 0,
                                                    patch: 1,
                                                },
                                                is_dependent: true,
                                            },
                                            details: Integer(
                                                IntegerDescriptor {
                                                    format: U8,
                                                    min_value: 0,
                                                    max_value: 127,
                                                },
                                            ),
                                        },
                                        sharing: Dedicated,
                                    },
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
                                    "an_I16_property": PropertyDescriptorUsage {
                                        description: "example string usage description",
                                        descriptor: PropertyDescriptor {
                                            header: TypeHeader {
                                                type_name: "simple_I16_Integer_Type",
                                                base_type: Integer,
                                                description: "Simple Integer (I16) Property Type description",
                                                version: SemanticVersion {
                                                    major: 0,
                                                    minor: 0,
                                                    patch: 1,
                                                },
                                                is_dependent: true,
                                            },
                                            details: Integer(
                                                IntegerDescriptor {
                                                    format: I16,
                                                    min_value: -32767,
                                                    max_value: 32767,
                                                },
                                            ),
                                        },
                                        sharing: Dedicated,
                                    },
                                    "an_I32_property": PropertyDescriptorUsage {
                                        description: "example string usage description",
                                        descriptor: PropertyDescriptor {
                                            header: TypeHeader {
                                                type_name: "simple_I32_Integer_Type",
                                                base_type: Integer,
                                                description: "Simple Integer (I32) Property Type description",
                                                version: SemanticVersion {
                                                    major: 0,
                                                    minor: 0,
                                                    patch: 1,
                                                },
                                                is_dependent: true,
                                            },
                                            details: Integer(
                                                IntegerDescriptor {
                                                    format: I32,
                                                    min_value: -2147483648,
                                                    max_value: 2147483648,
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
                                    "an_I8_property": PropertyDescriptorUsage {
                                        description: "example string usage description",
                                        descriptor: PropertyDescriptor {
                                            header: TypeHeader {
                                                type_name: "simple_I8_Integer_Type",
                                                base_type: Integer,
                                                description: "Simple Integer (I8) Property Type description",
                                                version: SemanticVersion {
                                                    major: 0,
                                                    minor: 0,
                                                    patch: 1,
                                                },
                                                is_dependent: true,
                                            },
                                            details: Integer(
                                                IntegerDescriptor {
                                                    format: I8,
                                                    min_value: -127,
                                                    max_value: 127,
                                                },
                                            ),
                                        },
                                        sharing: Dedicated,
                                    },
                                },
                            },
                        },
                    ),
                },
                sharing: Dedicated,
            },
        },
    },
}


```
