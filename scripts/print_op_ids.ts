import type { OpenAPIV3_1 } from "npm:openapi-types";

const path = "./spec/openapi.json";

const toCamelCase = (name: string): string => {
	const segments = name.split(/[^a-zA-Z0-9]/g);
	const result = segments
		.map((segment, index) =>
			index === 0
				? segment.toLowerCase()
				: segment.charAt(0).toUpperCase() + segment.slice(1).toLowerCase(),
		)
		.join("");
	return result;
};

const extractOpIds = async (): Promise<string[]> => {
	const fileContent = await Deno.readTextFile(path);
	const openapiSpec = JSON.parse(fileContent) as OpenAPIV3_1.Document;

	const paths = openapiSpec.paths;
	if (!paths) {
		throw Error("No paths found in OpenAPI spec.");
	}

	const operations = Object.values(paths)
		.filter((a) => a !== undefined)
		.flatMap((methods) => Object.values(methods));

	return operations
		.map((operation) => {
			if (typeof operation === "object" && "operationId" in operation) {
				return operation.operationId;
			}
			if (typeof operation === "string") {
				return operation;
			}
		})
		.filter((a) => a !== undefined);
};

for (const operationId of await extractOpIds()) {
	console.log(toCamelCase(operationId));
}
