# OpenAI Assistants

## Needed dependencies

- `curl`
- `jq` (optional)

## Tips

- For streaming scenarios `jq` doesn't work, since it's not technically JSON that comes back (as a single response). In that case is better to dump into a file by adding ` > dump.json` after the `curl` call
- Be mindful of trailing space after `\` (that took me too much time to figure out)

## Curl template

Notice the authentication is different depending on whether we are using Azure or not.

### 
