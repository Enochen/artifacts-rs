package com.artifacts.codegen;

import org.openapitools.codegen.CodegenModel;
import org.openapitools.codegen.CodegenProperty;
import org.openapitools.codegen.CodegenType;
import org.openapitools.codegen.SupportingFile;
import org.openapitools.codegen.languages.RustClientCodegen;

import io.swagger.v3.oas.models.OpenAPI;

import java.util.Set;
import java.util.LinkedHashSet;

public class ArtifactsCodegen extends RustClientCodegen {

    protected String sourceFolder = "src";
    protected String apiVersion = "1.0.0";

    @Override
    public CodegenType getTag() {
        return CodegenType.CLIENT;
    }

    @Override
    public String getName() {
        return "artifacts-codegen";
    }

    @Override
    public String getHelp() {
        return "Generates a client library for Artifacts code.";
    }

    public ArtifactsCodegen() {
        super();
        embeddedTemplateDir = templateDir = "template";
        supportingFiles.add(new SupportingFile("traits.mustache", "src", "traits.rs"));
    }

    @Override
    public void preprocessOpenAPI(OpenAPI openAPI) {
        validateOperationIds(openAPI);
        super.preprocessOpenAPI(openAPI);
    }

    private void validateOperationIds(OpenAPI openAPI) {
        if (openAPI.getPaths() == null) {
            return;
        }

        Set<String> specOperationIds = openAPI.getPaths().values().stream()
                .flatMap(path -> path.readOperationsMap().values().stream())
                .map(op -> removeNonNameElementToCamelCase(op.getOperationId()))
                .collect(java.util.stream.Collectors.toCollection(LinkedHashSet::new));

        Set<String> mappedOperationIds = operationIdNameMapping.keySet();

        assertSetsEqual(specOperationIds, mappedOperationIds);
    }

    private void assertSetsEqual(Set<String> expected, Set<String> actual) {
        Set<String> missing = new LinkedHashSet<>(expected);
        missing.removeAll(actual);

        Set<String> extra = new LinkedHashSet<>(actual);
        extra.removeAll(expected);

        if (!missing.isEmpty() || !extra.isEmpty()) {
            throw new RuntimeException(
                    String.format(
                            "Set mismatch:\nmissing=%s\nextra=%s\nactual=%s",
                            missing,
                            extra,
                            expected));
        }
    }

    @Override
    public void postProcessModelProperty(CodegenModel model, CodegenProperty property) {
        super.postProcessModelProperty(model, property);

        if (property.baseType.equals("CooldownSchema")) {
            model.vendorExtensions.put("x-cooldown-field", property.name);
        }

        if (property.baseType.equals("CharacterSchema")) {
            model.vendorExtensions.put("x-character-field", property.name);
        }

        if (property.name.equals("data") && model.vars.size() == 1) {
            model.vendorExtensions.put("x-data-type", property);
        }

        if (property.isAnyType || (property.items != null && property.items.isAnyType)) {
            property.vendorExtensions.put("x-unknown-value", true);
        }
    }
}
