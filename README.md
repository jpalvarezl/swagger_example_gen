## Swagger example generator tool

This is (will be) a tool used to generate examples for our Azure OpenAI REST API definitions

## Goals
- Not too concerned with good types. If can get away with a single request model `Prompts` for every request, that is good enough
- We dump JSON files ready to be used in the [REST API repo](https://github.com/Azure/azure-rest-api-specs-pr/)
- Maaaaybe at somepoint we can pass the response model structure as a `yml` and craft it to look nice
