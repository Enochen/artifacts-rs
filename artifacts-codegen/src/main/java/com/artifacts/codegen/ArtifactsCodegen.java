package com.artifacts.codegen;

import org.openapitools.codegen.CodegenModel;
import org.openapitools.codegen.CodegenOperation;
import org.openapitools.codegen.CodegenParameter;
import org.openapitools.codegen.CodegenProperty;
import org.openapitools.codegen.CodegenType;
import org.openapitools.codegen.SupportingFile;
import org.openapitools.codegen.languages.RustClientCodegen;
import org.openapitools.codegen.model.ModelMap;
import org.openapitools.codegen.model.OperationsMap;
import org.openapitools.codegen.utils.ModelUtils;

import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.media.Schema;
import io.swagger.v3.oas.models.media.IntegerSchema;

import java.util.Set;
import io.swagger.v3.oas.models.parameters.RequestBody;
import java.util.LinkedHashSet;
import java.util.List;

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

        if (openAPI.getComponents() != null && openAPI.getComponents().getSchemas() != null) {
            for (Schema<?> schema : openAPI.getComponents().getSchemas().values()) {
                if (schema.getProperties() != null) {
                    Schema<?> pathProp = schema.getProperties().get("path");
                    if (pathProp != null && ModelUtils.isArraySchema(pathProp)) {
                        Schema<?> items = pathProp.getItems();
                        if (items != null && items.getMinItems() != null
                                && items.getMinItems().equals(items.getMaxItems())) {
                            items.setItems(new IntegerSchema());
                        }
                    }
                }
            }
        }

        super.preprocessOpenAPI(openAPI);
    }

    private void validateOperationIds(OpenAPI openAPI) {
        if (openAPI.getPaths() == null) {
            return;
        }

        Set<String> specOperationIds = openAPI.getPaths().values().stream()
                .flatMap(path -> path.readOperationsMap().values().stream())
                .map(op -> op.getOperationId())
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
    public String toModelFilename(String name) {
        String filename = super.toModelFilename(name);

        // Remove trailing underscores
        filename = filename.replaceAll("_+$", "");

        return filename;
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

        if (property.isArray && property.complexType != null && property.complexType.equals("CharacterSchema")) {
            model.vendorExtensions.put("x-characters-field", property.name);
        }

        if (property.name.equals("data") && model.vars.size() == 1) {
            model.vendorExtensions.put("x-data-type", property);
        }

        if (property.isAnyType || property.isFreeFormObject
                || (property.items != null && property.items.isAnyType)
                || (property.items != null && property.items.isFreeFormObject)) {
            property.vendorExtensions.put("x-unknown-value", true);
        }
    }

    @Override
    public OperationsMap postProcessOperationsWithModels(OperationsMap objs, List<ModelMap> allModels) {
        OperationsMap map = super.postProcessOperationsWithModels(objs, allModels);
        for (CodegenOperation operation : map.getOperations().getOperation()) {
            if (operation.responses.stream().filter(response -> response.is4xx || response.is5xx)
                    .allMatch(response -> response.baseType.equals("ErrorResponseSchema"))) {
                operation.vendorExtensions.put("x-error-field", "models::ErrorResponseSchema");
            }
        }
        return map;
    }
}
