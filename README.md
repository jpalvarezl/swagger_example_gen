## Swagger example generator tool

This is (will be) a tool used to generate examples for our Azure OpenAI REST API definitions

## Setup

### Environment variables

```bash
AZURE_OPENAI_ENDPOINT="https://..."
AZURE_OPENAI_KEY=...
OPENAI_ENDPOINT="https://api.openai.com/v1"
OPENAI_KEY=...
```

### Dependencies

- `curl`
- `jq` (optional): if you want to pretty print to console, it's nice.
- To run `*.http` files, use the VSCode extension [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client)

## Goals
- Not too concerned with good types. If can get away with a single request model `Prompts` for every request, that is good enough
- We dump JSON files ready to be used in the [REST API repo](https://github.com/Azure/azure-rest-api-specs-pr/)
- Maaaaybe at somepoint we can pass the response model structure as a `yml` and craft it to look nice
