let context = #{};

context += prompt("Project Prefix:", "project-prefix", #{
    default_value: "",
    cases: CasedIdentityCasedValue(PROGRAMMING_CASES),
});

context += prompt("Project Suffix:", "project-suffix", #{
    defaults_with: "Service",
    cases: CasedIdentityCasedValue(PROGRAMMING_CASES),
});

context += set("project-name", render("{{ project-prefix }}-{{project-suffix}}", context), #{
    cases: [
        CasedIdentityCasedValue(PROGRAMMING_CASES),
        FixedKeyCasedValue("project-title", TitleCase),
    ],
});

context += prompt("Service Port:", "service-port", #{
    defaults_with: 8080,
    type: Int,
    min: 1024,
    max: 65534,
});

context += render(Archetype("model-casing"), ANSWERS + #{
    default_model: context["project-prefix"],
});

if switch_enabled("debug-context") {
    display(as_yaml(context));
}

render(Directory("contents/base"), context);
for entity in context.model.entities.values() {
    render(Directory("contents/entity_persistence"), context + entity);
}

if type_of(context.applications) == "map" {
    let settings = archetect::archetype::settings();
    let path = Path(`${context["project-name"]}/specs/clients`);
    let archetype = Archetype("generate-protobuf");
    for application in context.applications.values() {
        render(archetype, path, application, settings);

        if type_of(application.model.entities) == "map" {
            let directory = Directory("contents/entity_graphql");
            for entity in application.model.entities.values() {
                let context = context + entity + #{
                    application: application,
                };
                render(directory, context);
            }
        }
    }
}

// return context
context